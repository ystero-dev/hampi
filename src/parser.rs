//! Parser for an ASN.1 module

use crate::error::Error;
use crate::structs::{LineColumn, Span, Token, TokenType};

// Parses Begin/End of round curly brackets.
//
// Note: square brackets need a special treatment due to "[[" and "]]"
fn parse_bracket_token(token: char, line: usize, start: usize) -> Result<Token, Error> {
    let mut token_type: TokenType;
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
    let mut column = 0 as usize;
    let mut last_columns = 0 as usize;
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let mut buffer = String::new();
        let read_line = input.read_line(&mut buffer).unwrap();
        if read_line == 0 {
            break;
        }
        let chars: Vec<char> = buffer.chars().collect();
        column = 0;
        loop {
            if column == chars.len() {
                last_columns = column;
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

    use super::*;

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
    fn parse_curly_braces() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(b"{}"));
        let result = crate::parser::parse(reader);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.len() == 2, "{:#?}", tokens);
    }
}
