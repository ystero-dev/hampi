//! Parser for an ASN.1 module

use crate::error::Error;
use crate::structs::{LineColumn, Span, Token, TokenType};

fn parse_maybe_comment(
    chars: &[char], // From the first "--"
    line: usize,
    begin: usize,
) -> Result<(Option<Token>, usize), Error> {
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
        // No comment found
        Err(Error::ParseError)
    } else {
        let start = LineColumn::new(line, begin);
        let end = LineColumn::new(line, consumed);
        let token_string = chars[2..consumed]
            .iter()
            .collect::<String>()
            .trim_end_matches("--")
            .trim()
            .to_string();
        eprintln!(
            "last_idx: {}, name: {}, consumed: {}",
            last_idx.unwrap(),
            token_string,
            consumed
        );
        Ok((
            Some(Token {
                r#type: TokenType::Comment,
                span: Span::new(start, end),
                name: token_string,
            }),
            consumed,
        ))
    }
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
                    }
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
}
