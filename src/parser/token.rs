use crate::common::error::ParseError;
use crate::common::error::Result;
use crate::common::span::Range;
use crate::parser::token_kind::TokenKind;
use logos::{Lexer, Logos};

#[derive(Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub source: &'a str,
    pub kind: TokenKind,
    pub span: Range,
    pub pos: usize,
}

impl<'a> Token<'a> {
    pub fn new_eoi(source: &'a str) -> Self {
        Token {
            source,
            kind: TokenKind::EOI,
            span: (source.len()..source.len()).into(),
            pos: 0,
        }
    }

    pub fn text(&self) -> &'a str {
        &self.source[std::ops::Range::from(self.span)]
    }

    pub fn get_trim_start_end_text(&self, sep: char) -> &'a str {
        self.text()
            .trim_start_matches(|c| c == sep)
            .trim_end_matches(|c| c == sep)
    }
}

impl<'a> std::fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.span)
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
    prev_token: Option<TokenKind>,
    eoi: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            lexer: TokenKind::lexer(source),
            eoi: false,
            prev_token: None,
        }
    }

    pub fn contains_token(query: &str, target_kind: TokenKind) -> bool {
        let mut tokenizer = Tokenizer::new(query);
        while let Some(Ok(token)) = tokenizer.next() {
            if token.kind == target_kind {
                return true;
            }
        }
        false
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(rs) => {
                match rs {
                    // Ok(TokenKind::Error) => {
                    //     let span = Some((self.lexer.span().start..self.source.len()).into());
                    //     Some(Err(ParseError(
                    //         span,
                    //         "unable to recognize the rest tokens(token custom error)".to_string(),
                    //     )))
                    // }
                    Ok(kind) => {
                        // Skip hint-like comment that is in the invalid position.
                        if !matches!(
                            self.prev_token,
                            Some(
                                TokenKind::INSERT
                                    | TokenKind::SELECT
                                    | TokenKind::REPLACE
                                    | TokenKind::UPDATE
                                    | TokenKind::DELETE
                            )
                        ) && kind == TokenKind::HintPrefix
                        {
                            loop {
                                match self.next() {
                                    // Hint-like comment ended. Return the next token.
                                    Some(Ok(token)) if token.kind == TokenKind::HintSuffix => {
                                        return self.next();
                                    }
                                    // Do not skip EOI.
                                    Some(Ok(token)) if token.kind == TokenKind::EOI => {
                                        return Some(Ok(token));
                                    }
                                    // In the comment, skip the contents.
                                    Some(Ok(_)) => continue,
                                    Some(Err(err)) => return Some(Err(err)),
                                    None => return None,
                                }
                            }
                        }
                        self.prev_token = Some(kind);
                        Some(Ok(Token {
                            source: self.source,
                            kind,
                            span: self.lexer.span().into(),
                            pos: 0,
                        }))
                    }
                    Err(_) => {
                        let span = Some((self.lexer.span().start..self.source.len()).into());
                        Some(Err(ParseError(
                            span,
                            "unable to recognize the rest tokens(token core error)".to_string(),
                        )))
                    }
                }
            }
            None if !self.eoi => {
                self.eoi = true;
                Some(Ok(Token::new_eoi(self.source)))
            }
            None => None,
        }
    }
}
