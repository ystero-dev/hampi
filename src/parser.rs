#![allow(dead_code)]
#![allow(unused_variables)]
//! Parser for an ASN.1 module

use crate::error::Error;
use crate::structs::{LineColumn, Span, Token, TokenType};

// Parse Identifier or a Keyword
//
// This parses all types of identifiers including references and ASN.1 keywords. Returns the
// appropriate type of the token and bytes consumed.
fn parse_identifier_or_keyword(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    Err(Error::ParseError)
}

// Parse Range ".." or Extension  "..."
fn parse_range_or_extension(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let (token_type, consumed) = if chars[1] == '.' {
        if chars[2] == '.' {
            (TokenType::Extension, 3)
        } else {
            (TokenType::RangeSeparator, 2)
        }
    } else {
        return Err(Error::ParseError);
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
fn parse_assignment_or_colon(
    chars: &[char],
    line: usize,
    begin: usize,
) -> Result<(Token, usize), Error> {
    let (token_type, consumed) = if chars[1] == ':' && chars[2] == '=' {
        (TokenType::Assignment, 3)
    } else {
        (TokenType::Colon, 1)
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

// Parses either a Square bracket begin or Sequence Extension
fn parse_seq_extension_or_square_brackets(
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

// Parses Begin/End of round curly brackets.
//
// Note: square brackets need a special treatment due to "[[" and "]]"
fn parse_bracket_token(token: char, line: usize, start: usize) -> Result<Token, Error> {
    let token_type: TokenType;
    match token {
        '{' => token_type = TokenType::CurlyBegin,
        '}' => token_type = TokenType::CurlyEnd,
        '(' => token_type = TokenType::RoundBegin,
        ')' => token_type = TokenType::RoundEnd,
        _ => return Err(Error::ParseError),
    }
    Ok(Token {
        r#type: token_type,
        span: Span::new(
            LineColumn::new(line, start),
            LineColumn::new(line, start + 1),
        ),
        text: token.to_string(),
    })
}

// Parses a comment. The comment will be of the form -
// -- Some Comment \n or
// -- Some Comment -- or
// -- Some Comment EOF (Note: last is a special case and not exactly confirming to standards.)
//
// A pathologically crafted line of size greater than usize could cause this function to panic.
// Which we don't expect to see in real life normally.
fn parse_maybe_comment(
    chars: &[char], // From the first "--"
    line: usize,
    begin: usize,
) -> Result<(Option<Token>, usize), Error> {
    eprintln!("chars[0]: {}, chars[1]: {}", chars[0], chars[1]);
    if chars[1] != '-' {
        return Ok((None, 0));
    }
    let mut consumed = 2; // initial "--"
    let mut last_idx: Option<usize> = None;

    // May be a comment like -- Comment --
    for (idx, window) in chars[2..].windows(2).enumerate() {
        if window[0] == '-' && window[1] == '-' {
            last_idx = Some(idx);
            consumed += idx + 2; // --123-- : idx: 3 consumed: 7
            break;
        }
    }

    if last_idx.is_none() {
        // May be comment is like // Comment \n
        last_idx = chars[2..].iter().position(|&x| x == '\n');
        if last_idx.is_some() {
            consumed += last_idx.unwrap() + 1; // --123\n : idx: 0 consumed: 6
        }
    }

    if last_idx.is_none() {
        // Neither "--" nor '\n' found. consume everything. (may be last line.)
        consumed = chars.len();
        last_idx = Some(consumed);
    }

    let text = chars[..consumed]
        .iter()
        .collect::<String>()
        .trim_start_matches("--")
        .trim_end_matches("--")
        .trim()
        .to_string();
    eprintln!(
        "last_idx: {}, text: {}, consumed: {}",
        last_idx.unwrap(),
        text,
        consumed
    );
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

fn parse<T>(mut input: T) -> Result<Vec<Token>, Error>
where
    T: std::io::BufRead,
{
    let mut line = 1;
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let mut buffer = String::new();
        let read_line = input.read_line(&mut buffer).unwrap();
        if read_line == 0 {
            break;
        }
        let chars: Vec<char> = buffer.chars().collect();
        let mut column = 0 as usize;
        loop {
            if column == chars.len() {
                break;
            }
            let c = chars[column];
            match c {
                ' ' | '\t' => {
                    column += 1;
                }
                '-' => {
                    let (token, consumed) = parse_maybe_comment(&chars[column..], line, column)?;
                    if token.is_some() {
                        tokens.push(token.unwrap());
                    } else {
                        // FIXME: Try parsing a negative number
                    }
                    column += consumed;
                }
                '{' | '}' | '(' | ')' => {
                    let token = parse_bracket_token(chars[column], line, column)?;
                    tokens.push(token);
                    column += 1;
                }
                '[' | ']' => {
                    let (token, consumed) =
                        parse_seq_extension_or_square_brackets(&chars[column..], line, column)?;
                    tokens.push(token);
                    column += consumed;
                }
                ':' => {
                    let (token, consumed) =
                        parse_assignment_or_colon(&chars[column..], line, column)?;
                    tokens.push(token);
                    column += consumed;
                }
                '.' => {
                    let (token, consumed) =
                        parse_range_or_extension(&chars[column..], line, column)?;
                    tokens.push(token);
                    column += consumed;
                }
                _ => {
                    column += 1;
                }
            }
        }
        line += 1
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {

    #[test]
    fn consume_parse() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b"Hello World!"));
        let result = crate::parser::parse(reader);
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 0);
    }

    #[test]
    fn parse_comment_two_lines() {
        let reader =
            std::io::BufReader::new(std::io::Cursor::new(b"Hello World!\n-- Some comment --\n"));
        let result = crate::parser::parse(reader);
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 1);
    }

    #[test]
    fn parse_two_comments() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(
            b" -- Hello World!\n-- Some comment --\n",
        ));
        let result = crate::parser::parse(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
    }

    #[test]
    fn parse_comment_no_trailing_newline() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b" -- Hello World!"));
        let result = crate::parser::parse(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 1, "{:#?}", tokens);
    }

    #[test]
    fn parse_small_tokens() {
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
        ];
        for test_case in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(test_case.input));
            let result = crate::parser::parse(reader);
            assert_eq!(result.is_ok(), test_case.success);
            if result.is_ok() {
                let tokens = result.unwrap();
                assert!(tokens.len() == test_case.count, "{:#?}", tokens);
            }
        }
    }
}
