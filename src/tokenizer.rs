#![allow(dead_code)]
#![allow(unused_variables)]
//! Tokenizer for an ASN.1 module

use crate::error::Error;
use crate::token_types::TokenType;

// Keywords
const KEYWORDS: &'static [&'static str] = &[
    "ABSENT",
    "ABSTRACT-SYNTAX",
    "ALL",
    "APPLICATION",
    "AUTOMATIC",
    "BEGIN",
    "BIT",
    "BMPString",
    "BOOLEAN",
    "BY",
    "CHARACTER",
    "CHOICE",
    "CLASS",
    "COMPONENT",
    "COMPONENTS",
    "CONSTRAINED",
    "CONTAINING",
    "DEFAULT",
    "DEFINITIONS",
    "EMBEDDED",
    "ENCODED",
    "END",
    "ENUMERATED",
    "EXCEPT",
    "EXPLICIT",
    "EXPORTS",
    "EXTENSIBILITY",
    "EXTERNAL",
    "FALSE",
    "FRom",
    "GeneralizedTime",
    "GeneralString",
    "GraphicString",
    "IA5String",
    "IDENTIFIER",
    "IMPLIED",
    "IMPLICIT",
    "IMPORTS",
    "INCLUDES",
    "INSTANCE",
    "INTEGER",
    "INTERSECTION",
    "ISO646String",
    "MAX",
    "MIN",
    "MINUS-INFINITY",
    "NULL",
    "NumericString",
    "OBJECT",
    "ObjectDescriptor",
    "OCTET",
    "OF",
    "OPTIONAL",
    "PATTERN",
    "PDV",
    "Plus-Infinity",
    "PRESENT",
    "PrintableString",
    "PRIVATE",
    "REAL",
    "RELATIVE-OID",
    "SEQUENCE",
    "SET",
    "SIZE",
    "STRING",
    "SYNTAX",
    "T61String",
    "TAGS",
    "TeletexString",
    "TRUE",
    "TYPE-IDENTIFIER",
    "UNION",
    "UNIQUE",
    "UNIVERSAL",
    "UniversalString",
    "UTCTime",
    "UTF8String",
    "VideotexString",
    "VisibleString",
    "WITH",
];

/// Line and Column in the source where the token begins.
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct LineColumn {
    line: usize,
    column: usize,
}

impl std::fmt::Display for LineColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.column)
    }
}

impl LineColumn {
    fn new(line: usize, column: usize) -> Self {
        LineColumn { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

/// Span of a Token in the ASN Source file.
#[derive(Debug, Clone)]
pub struct Span {
    start: LineColumn,
    end: LineColumn,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Start:: {}, End:: {}", self.start, self.end)
    }
}

impl Span {
    fn new(start: LineColumn, end: LineColumn) -> Self {
        Span { start, end }
    }

    pub(crate) fn start(&self) -> LineColumn {
        self.start
    }

    pub(crate) fn end(&self) -> LineColumn {
        self.end
    }
}

/// A parsed token before AST is created.
///
/// Going through an ASN.1 module source results in a vector of parsed tokens of appropriate types.
/// Each parsed token contains the 'type', where it is found in the source ('span') and the actual
/// token string.
///
/// The tokens are then used by the Parser to 'resolve' type and value definitions that generates
/// the AST.
#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub span: Span,
    pub text: String,
}

impl Token {
    create_is_tokentype_fns! {
        (is_curly_begin, TokenType::CurlyBegin),
        (is_curly_end, TokenType::CurlyEnd),
        (is_round_begin, TokenType::RoundBegin),
        (is_round_end, TokenType::RoundEnd),
        (is_exception_marker, TokenType::ExceptionMarker),
        (is_square_begin, TokenType::SquareBegin),
        (is_square_end, TokenType::SquareEnd),
        (is_seq_extension_begin, TokenType::SeqExtensionBegin),
        (is_seq_extension_end, TokenType::SeqExtensionEnd),
        (is_extension, TokenType::Extension),
        (is_range_separator, TokenType::RangeSeparator),
        (is_assignment, TokenType::Assignment),
        (is_colon, TokenType::Colon),
        (is_semicolon, TokenType::SemiColon),
        (is_identifier, TokenType::Identifier),
        (is_keyword, TokenType::Keyword),
        (is_comment, TokenType::Comment),
        (is_and_identifier, TokenType::AndIdentifier),
        (is_numeric, TokenType::NumberInt),
        (is_bitstring, TokenType::BitString),
        (is_hexstring, TokenType::HexString),
        (is_tstring, TokenType::TString),
        (is_dot, TokenType::Dot),
        (is_comman, TokenType::Comma),
        (is_set_union, TokenType::SetUnion),
        (is_set_intersection, TokenType::SetIntersection),
        (is_at_component_list, TokenType::AtComponentIdList),
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn is_given_keyword(&self, keyword: &str) -> bool {
        self.is_keyword() && self.text == keyword
    }
}

// Get string token.
fn get_string_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize, usize, usize), Error> {
    let mut last: Option<usize> = None;

    if chars.len() == 1 {
        return Err(Error::TokenizeError(0, line, begin));
    }

    let mut i = 1;
    loop {
        // " " " " .
        // 0 1 2 3 4 (len = 5)
        // i = 1
        // i = 3
        //
        // " a " . .
        // 0 1 2 3 4 (len = 5)
        // i = 1
        // i = 2
        //
        // " " " .
        // 0 1 2 3 (len = 4)
        // i = 1
        // i = 3
        if i >= chars.len() - 1 {
            if i == chars.len() - 1 {
                if chars[i] == '"' {
                    last = Some(i);
                }
            }
            break;
        }
        if chars[i] == '"' {
            if chars[i + 1] == '"' {
                i += 2;
            } else {
                last = Some(i);
                break;
            }
        } else {
            i += 1;
        }
    }

    // If we didn't find the last '"'
    if last.is_none() {
        return Err(Error::TokenizeError(5, line, begin));
    }

    let consumed = last.unwrap() + 1;

    let mut text = chars[..consumed].iter().collect::<String>();
    let lines = text.lines().count() - 1;
    let last_line = text.lines().last().unwrap();
    let end_column = if lines > 0 {
        last_line.len()
    } else {
        begin + consumed
    };
    text = text
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("");

    Ok((
        Token {
            r#type: TokenType::TString,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line + lines, end_column),
            ),
            text,
        },
        consumed,
        lines,
        end_column,
    ))
}
// Get bit string or hex string
fn get_bit_or_hex_string_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize, usize, usize), Error> {
    if chars.len() == 1 {
        return Err(Error::TokenizeError(6, line, begin));
    }

    let last = chars[1..].iter().position(|&c| c == '\'');
    if last.is_none() {
        // No matching '\'' found till the end of the string. Clearly an error.
        return Err(Error::TokenizeError(7, line, begin));
    }
    let mut consumed = last.unwrap() + 1 + 1;
    if consumed == chars.len() {
        // Matching'\'' found, but the string ends, Error.
        return Err(Error::TokenizeError(8, line, begin));
    }

    let c = chars[consumed];
    let token_type = match c.to_lowercase().to_string().as_str() {
        "h" => TokenType::HexString,
        "b" => TokenType::BitString,
        _ => {
            return Err(Error::TokenizeError(9, line, begin));
        }
    };

    let mut text = chars[..consumed].iter().collect::<String>();
    let lines = text.lines().count() - 1;
    let last_line = text.lines().last().unwrap();
    let end_column = if lines > 0 {
        last_line.len()
    } else {
        begin + consumed
    };
    text = text.replace(char::is_whitespace, "");

    if token_type == TokenType::BitString && text.replace(&['0', '1', '\''][..], "").len() != 0 {
        return Err(Error::TokenizeError(10, line, begin));
    }

    if token_type == TokenType::HexString
        && !text.chars().all(|c| c.is_ascii_hexdigit() || c == '\'')
    {
        return Err(Error::TokenizeError(11, line, begin));
    }

    consumed += 1; // last 'h' or 'b'

    Ok((
        Token {
            r#type: token_type,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line + lines, end_column), // FIXME: This span may be wrong, but ignore right now
            ),
            text,
        },
        consumed,
        lines,
        end_column,
    ))
}

// Get at and component ID list something like @.id or @component.id
fn get_at_component_id_list(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    if chars.len() == 1 {
        return Err(Error::TokenizeError(12, line, begin));
    }

    let mut consumed = 1;
    let last = chars[1..]
        .iter()
        .position(|&x| !(x.is_ascii_alphanumeric() || x == '-' || x == '.'));
    if last.is_none() {
        consumed += chars[1..].len();
    } else {
        consumed += last.unwrap();
    }

    // Identifier should not end with a '-'
    if ['.', '-'].iter().any(|&c| c == chars[consumed - 1]) {
        return Err(Error::TokenizeError(13, line, begin));
    }
    Ok((
        Token {
            r#type: TokenType::AtComponentIdList,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text: chars[..consumed].iter().collect::<String>(), // include the sign as well
        },
        consumed,
    ))
}
// Get token for a number Integer or Real
fn get_number_token(chars: &[char], line: usize, begin: usize) -> Result<(Token, usize), Error> {
    let neg = (chars[0] == '-') as usize;

    if neg > 0 && chars.len() == 1 {
        return Err(Error::TokenizeError(14, line, begin));
    }

    let mut consumed = neg;
    let last = chars[neg..].iter().position(|&x| !x.is_numeric());
    if last.is_none() {
        consumed += chars[neg..].len();
    } else {
        consumed += last.unwrap();
    }

    Ok((
        Token {
            r#type: TokenType::NumberInt,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text: chars[..consumed].iter().collect::<String>(), // include the sign as well
        },
        consumed,
    ))
}

// Get token for Identifier or a Keyword
//
// This parses all types of identifiers including references and ASN.1 keywords. Returns the
// appropriate type of the token and bytes consumed.
// This also processes identifiers of the form `&identifer` or `&Identifier`.
fn get_identifier_or_keyword_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let and = (chars[0] == '&') as usize;

    if and > 0 && chars.len() == 1 {
        return Err(Error::TokenizeError(15, line, begin));
    }

    let mut consumed = and;
    let last = chars[and..]
        .iter()
        .position(|&x| !(x.is_ascii_alphanumeric() || x == '-'));
    if last.is_none() {
        consumed += chars[and..].len();
    } else {
        consumed += last.unwrap();
    }

    // Identifier should not end with a '-'
    if chars[consumed - 1] == '-' {
        return Err(Error::TokenizeError(16, line, begin));
    }

    // Free standing '&' this is an error.
    if and > 0 && consumed == 1 {
        return Err(Error::TokenizeError(17, line, begin));
    }

    let text = chars[..consumed].iter().collect::<String>();
    if text.find("--").is_some() {
        return Err(Error::TokenizeError(18, line, begin));
    }

    let token_type = if and > 0 {
        TokenType::AndIdentifier
    } else {
        if KEYWORDS.iter().any(|&kw| text == kw) {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        }
    };

    Ok((
        Token {
            r#type: token_type,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text,
        },
        consumed,
    ))
}

// Get token for Range ".." or Extension  "..."
fn get_range_or_extension_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let (token_type, consumed) = if chars.len() == 1 {
        (TokenType::Dot, 1)
    } else if chars.len() == 2 {
        if chars[1] == '.' {
            (TokenType::RangeSeparator, 2)
        } else {
            (TokenType::Dot, 1)
        }
    } else {
        if chars[1] == '.' {
            if chars[2] == '.' {
                (TokenType::Extension, 3)
            } else {
                (TokenType::RangeSeparator, 2)
            }
        } else {
            (TokenType::Dot, 1)
        }
    };

    Ok((
        Token {
            r#type: token_type,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text: chars[..consumed].iter().collect::<String>(),
        },
        consumed,
    ))
}

// Parse either an assignment token "::=" pr a single ':'
fn get_assignment_or_colon_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let (token_type, consumed) = if chars.len() == 1 {
        (TokenType::Colon, 1)
    } else if chars.len() == 2 {
        if chars[1] == ':' {
            return Err(Error::TokenizeError(19, line, begin));
        } else {
            (TokenType::Colon, 1)
        }
    } else {
        if chars[1] == ':' {
            if chars[2] == '=' {
                (TokenType::Assignment, 3)
            } else {
                return Err(Error::TokenizeError(20, line, begin));
            }
        } else {
            (TokenType::Colon, 1)
        }
    };

    Ok((
        Token {
            r#type: token_type,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text: chars[..consumed].iter().collect::<String>(),
        },
        consumed,
    ))
}

// Gets either a Square bracket or Sequence Extension
//
// This gives all the tokens '[[' or ']]' or '[' or ']'
fn get_seq_extension_or_square_brackets_token(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let (token_type, consumed) = if chars[0] == '[' {
        if chars[1] == '[' {
            (TokenType::SeqExtensionBegin, 2)
        } else {
            (TokenType::SquareBegin, 1)
        }
    } else {
        if chars[1] == ']' {
            (TokenType::SeqExtensionEnd, 2)
        } else {
            (TokenType::SquareEnd, 1)
        }
    };
    Ok((
        Token {
            r#type: token_type,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, begin + consumed),
            ),
            text: chars[..consumed].iter().collect::<String>(),
        },
        consumed,
    ))
}

// Gets Begin/End of round/curly brackets.
//
// Note: square brackets need a special treatment due to "[[" and "]]"
fn get_single_char_token(token: char, line: usize, begin: usize) -> Result<Token, Error> {
    let token_type: TokenType;
    match token {
        '{' => token_type = TokenType::CurlyBegin,
        '}' => token_type = TokenType::CurlyEnd,
        '(' => token_type = TokenType::RoundBegin,
        ')' => token_type = TokenType::RoundEnd,
        '!' => token_type = TokenType::ExceptionMarker,
        ';' => token_type = TokenType::SemiColon,
        ',' => token_type = TokenType::Comma,
        '|' => token_type = TokenType::SetUnion,
        '^' => token_type = TokenType::SetIntersection,
        _ => return Err(Error::TokenizeError(21, line, begin)),
    }
    Ok(Token {
        r#type: token_type,
        span: Span::new(
            LineColumn::new(line, begin),
            LineColumn::new(line, begin + 1),
        ),
        text: token.to_string(),
    })
}

// Gets a comment. The comment will be of the form -
// -- Some Comment \n or
// -- Some Comment -- or
// -- Some Comment EOF (Note: last is a special case and not exactly confirming to standards.)
//
// A cleverly crafted pathologically long line could cause this function to panic.
// Which we don't expect to see in real life normally.
fn get_maybe_comment_token(
    chars: &[char], // From the first "--"
    line: usize,
    begin: usize,
) -> Result<(Option<Token>, usize), Error> {
    if chars[1] != '-' {
        return Ok((None, 0));
    }
    let mut consumed = 2; // initial "--"
    let mut last_idx: Option<usize> = None;

    // Search for Either '\n' or "--"
    for (idx, window) in chars[2..].windows(2).enumerate() {
        if window[0] == '\n' {
            last_idx = Some(idx);
            consumed += idx;
            break;
        }
        if window[0] == '-' && window[1] == '-' {
            last_idx = Some(idx);
            consumed += idx + 2; // --123-- : idx: 3 consumed: 7
            break;
        }
    }

    // Neither "--" nor '\n' found. consume everything. (may be last line.)
    if last_idx.is_none() {
        consumed = chars.len();
    }

    let text = chars[..consumed]
        .iter()
        .collect::<String>()
        //.trim_start_matches("--")
        //.trim_end_matches("--")
        //.trim()
        .to_string();

    Ok((
        Some(Token {
            r#type: TokenType::Comment,
            span: Span::new(
                LineColumn::new(line, begin),
                LineColumn::new(line, consumed),
            ),
            text,
        }),
        consumed,
    ))
}

/// Tokenize ASN file.
///
/// This function would work on any input that implements `std::io::Read` trait, but would work
/// mostly with files because this 'reads the input to end'. We look at the first character of a
/// non-whitespace sequence and then tokenize that into appropriate tokens.
pub fn tokenize<T>(mut input: T) -> Result<Vec<Token>, Error>
where
    T: std::io::Read,
{
    let mut line = 1;
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = Vec::new();
    let _ = input.read_to_end(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer).unwrap();
    let chars: Vec<char> = buffer.chars().collect();
    let mut column = 0 as usize;
    let mut processed = 0;
    let total_read = chars.len();
    loop {
        let c = chars[processed];
        match c {
            ' ' | '\t' => {
                processed += 1;
                column += 1;
            }
            '\n' => {
                line += 1;
                processed += 1;
                column = 0;
            }
            '-' => {
                let (token, consumed) = get_maybe_comment_token(&chars[processed..], line, column)?;
                if token.is_some() {
                    tokens.push(token.unwrap());
                    column += consumed;
                    processed += consumed;
                } else {
                    let (token, consumed) = get_number_token(&chars[processed..], line, column)?;
                    tokens.push(token);
                    column += consumed;
                    processed += consumed;
                }
            }
            '{' | '}' | '(' | ')' | '!' | ';' | ',' | '|' | '^' => {
                let token = get_single_char_token(chars[processed], line, column)?;
                tokens.push(token);
                column += 1;
                processed += 1;
            }
            '[' | ']' => {
                let (token, consumed) =
                    get_seq_extension_or_square_brackets_token(&chars[processed..], line, column)?;
                tokens.push(token);
                column += consumed;
                processed += consumed;
            }
            ':' => {
                let (token, consumed) =
                    get_assignment_or_colon_token(&chars[processed..], line, column)?;
                tokens.push(token);
                column += consumed;
                processed += consumed;
            }
            '.' => {
                let (token, consumed) =
                    get_range_or_extension_token(&chars[processed..], line, column)?;
                tokens.push(token);
                column += consumed;
                processed += consumed;
            }
            '&' | 'a'..='z' | 'A'..='Z' => {
                let (token, consumed) =
                    get_identifier_or_keyword_token(&chars[processed..], line, column)?;
                tokens.push(token);

                column += consumed;
                processed += consumed;
            }
            '0'..='9' => {
                let (token, consumed) = get_number_token(&chars[processed..], line, column)?;
                tokens.push(token);
                column += consumed;
                processed += consumed;
            }
            '@' => {
                let (token, consumed) =
                    get_at_component_id_list(&chars[processed..], line, column)?;
                tokens.push(token);
                column += consumed;
                processed += consumed;
            }
            '\'' => {
                let (token, consumed, l, c) =
                    get_bit_or_hex_string_token(&chars[processed..], line, column)?;
                tokens.push(token);
                processed += consumed;
                if l > 0 {
                    column = c;
                } else {
                    column += consumed;
                }
                line += l;
            }
            '"' => {
                let (token, consumed, l, c) = get_string_token(&chars[processed..], line, column)?;
                tokens.push(token);
                processed += consumed;
                if l > 0 {
                    column = c;
                } else {
                    column += consumed;
                }
                line += l;
            }
            // Zero width ....
            '\u{feff}' => {
                processed += 1;
            }
            _ => {
                panic!(
                    "Unsupported First character for a token: '{:?}'. Line: {}, Column: {}",
                    chars[processed], line, column
                );
            }
        }
        if processed == total_read {
            break;
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {

    #[test]
    fn tokenize_identifier_tokens() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b"Hello World!"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok(), "{:#?}", result.err());
        let tokens = result.unwrap();
        assert!(tokens.len() == 3, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_and_tokens() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b"&Id &id-IDentifier"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_comment_two_lines() {
        let reader =
            std::io::BufReader::new(std::io::Cursor::new(b"Hello World!\n-- Some comment --\n"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 4, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_two_comments() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(
            b" -- Hello World!\n-- Some comment --\n",
        ));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_comment_no_trailing_newline() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b" -- Hello World!"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 1, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_keywords() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b"  INTEGER ENUMERATED "));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
        assert!(tokens.iter().all(|t| t.is_keyword()));
    }

    #[test]
    fn tokenize_at_component_list() {
        let reader =
            std::io::BufReader::new(std::io::Cursor::new(b"@component.id-List @.another "));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_numbers() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b" 123456789 -123"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
        assert!(tokens.iter().all(|t| t.is_numeric()), "{:#?}", tokens);
    }

    #[test]
    fn tokenize_keyword_dot_andkeyword() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(
            b"ATTRIBUTE.&equality-match.&AssertionType",
        ));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 5, "{:#?}", tokens);
    }

    #[test]
    fn tokenize_range() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b" -123456789..-123"));
        let result = crate::parser::tokenize(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 3, "{:#?}", tokens);
        assert!(tokens[0].is_numeric(), "{:#?}", tokens[0]);
        assert!(tokens[1].is_range_separator(), "{:#?}", tokens[1]);
        assert!(tokens[2].is_numeric(), "{:#?}", tokens[2]);
    }

    #[test]
    fn tokenize_bitstring() {
        struct BitHexStringTestCase<'t> {
            input: &'t [u8],
            success: bool,
            span_end_line: usize,
        }
        let test_cases = vec![
            BitHexStringTestCase {
                input: b"'010101'b",
                success: true,
                span_end_line: 1,
            },
            BitHexStringTestCase {
                input: b"'010101'",
                success: false,
                span_end_line: 1,
            },
            BitHexStringTestCase {
                input: b"'010101'h",
                success: true,
                span_end_line: 1,
            },
            BitHexStringTestCase {
                input: b"'01 0101'b",
                success: true,
                span_end_line: 1,
            },
            BitHexStringTestCase {
                input: b"'01 0101'h",
                success: true,
                span_end_line: 1,
            },
            BitHexStringTestCase {
                input: b"'01 0101\n\t0101\n00'h",
                success: true,
                span_end_line: 2,
            },
        ];
        for t in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(t.input));
            let result = crate::parser::tokenize(reader);
            assert_eq!(result.is_ok(), t.success, "{:#?}", result.unwrap()[0]);
            if result.is_ok() {
                let tokens = result.unwrap();
                assert!(tokens.len() == 1, "{:#?}", tokens[0]);
            }
        }
    }

    #[test]
    fn tokenize_string() {
        struct TestTokenizeString<'t> {
            input: &'t [u8],
            len: usize,
            success: bool,
        }
        let test_cases = vec![
            TestTokenizeString {
                input: b"\"Foo Bar\n\tFoo-baz\"",
                len: 1,
                success: true,
            },
            TestTokenizeString {
                input: b"\"",
                len: 1,
                success: false,
            },
            TestTokenizeString {
                input: b"\"\"",
                len: 1,
                success: true,
            },
            TestTokenizeString {
                input: b"\"\"\"",
                len: 1,
                success: false,
            },
            TestTokenizeString {
                input: b"\"\"\"\" ",
                len: 1,
                success: true,
            },
            TestTokenizeString {
                //input: b"\"\"Some Quoted String\"\"x",
                input: b"\"\"\"a\"\"x\"",
                len: 1,
                success: true,
            },
            TestTokenizeString {
                input: b"\"a\"..\"z\"",
                len: 3,
                success: true,
            },
        ];
        for test_case in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(test_case.input));
            let result = crate::parser::tokenize(reader);
            assert_eq!(
                result.is_ok(),
                test_case.success,
                "{}",
                result.err().unwrap()
            );
            if result.is_ok() {
                let tokens = result.unwrap();
                assert!(tokens.len() == test_case.len, "{:#?}", tokens);
            }
        }
    }
    #[test]
    fn tokenize_small_tokens() {
        struct SmallTokenTestCase<'t> {
            input: &'t [u8],
            count: usize,
            success: bool,
        }
        let test_cases = vec![
            SmallTokenTestCase {
                input: b"{{}}",
                count: 4,
                success: true,
            },
            SmallTokenTestCase {
                input: b"[[{}]}",
                count: 5,
                success: true,
            },
            SmallTokenTestCase {
                input: b"[[]]",
                count: 2,
                success: true,
            },
            SmallTokenTestCase {
                input: b"..{...}",
                count: 4,
                success: true,
            },
            SmallTokenTestCase {
                input: b":(::=)",
                count: 4,
                success: true,
            },
            SmallTokenTestCase {
                input: b": ::=",
                count: 2,
                success: true,
            },
            SmallTokenTestCase {
                input: b": :: ",
                count: 2,
                success: false,
            },
            SmallTokenTestCase {
                input: b".",
                count: 1,
                success: true,
            },
            SmallTokenTestCase {
                input: b"..",
                count: 1,
                success: true,
            },
            SmallTokenTestCase {
                input: b". ",
                count: 1,
                success: true,
            },
            SmallTokenTestCase {
                input: b". . .. ",
                count: 3,
                success: true,
            },
            SmallTokenTestCase {
                input: b"...",
                count: 1,
                success: true,
            },
        ];
        for test_case in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(test_case.input));
            let result = crate::parser::tokenize(reader);
            assert_eq!(
                result.is_ok(),
                test_case.success,
                "{}",
                String::from_utf8(test_case.input.to_vec()).unwrap()
            );
            if result.is_ok() {
                let tokens = result.unwrap();
                assert!(tokens.len() == test_case.count, "{:#?}", tokens);
            }
        }
    }
}
