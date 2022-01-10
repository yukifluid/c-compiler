use std::collections::HashMap;
use regex::Regex;
use crate::{token::Token, config::Config};

pub struct Lexer<'a> {
    src: &'a str,
    re: HashMap<&'static str, Regex>,
    matched: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(config: &'a Config) -> Lexer<'a> {
        let mut re = HashMap::new();

        // トークンにマッチする正規表現の生成
        re.insert("OP" , Regex::new(r"^(\+|-)" ).unwrap());
        re.insert("NUM", Regex::new(r"^\d+").unwrap());
        re.insert("EOF", Regex::new(r"^$"  ).unwrap());

        Lexer {
            src: &config.src,
            re,
            matched: ""
        }
    } 

    fn is_match(&mut self, token: &'static str) -> bool {
        if let Some(matched) = self.re[token].find(self.src) {
            self.matched = &self.src[matched.start()..matched.end()];
            return true
        }
        false
    }

    pub fn lex(&mut self) -> Result<Token, &'static str> {
        let result = match self.src {
            _ if self.is_match("OP" ) => Result::Ok(Token::OP(&self.matched)),
            _ if self.is_match("NUM") => Result::Ok(Token::NUM(&self.matched)),
            _ if self.is_match("EOF") => Result::Ok(Token::EOF),
            _ => Result::Err("There are no tokens to match")
        };
        if let Ok(_) = result {
            // srcの更新
            self.src = &self.src[self.matched.len()..];
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::Config;
    use super::Lexer;
    use super::Token;

    #[test]
    fn lex_addition() {
        let config = Config { src: "1+2".to_string() };
        let mut lexer = Lexer::new(&config);
        assert_eq!(lexer.lex().unwrap(), Token::NUM("1"));
        assert_eq!(lexer.lex().unwrap(), Token::OP("+"));
        assert_eq!(lexer.lex().unwrap(), Token::NUM("2"));
        assert_eq!(lexer.lex().unwrap(), Token::EOF);
    }

    #[test]
    fn lex_substraction() {
        let config = Config { src: "5-3".to_string() };
        let mut lexer = Lexer::new(&config);
        assert_eq!(lexer.lex().unwrap(), Token::NUM("5"));
        assert_eq!(lexer.lex().unwrap(), Token::OP("-"));
        assert_eq!(lexer.lex().unwrap(), Token::NUM("3"));
        assert_eq!(lexer.lex().unwrap(), Token::EOF);
    }
}