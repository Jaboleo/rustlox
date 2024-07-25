#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

pub struct Token {
    pub message: String,
    pub ttype: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: i32,
}

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn get_char(&self, idx: usize) -> char {
        self.source.chars().nth(idx).unwrap()
    }

    fn is_at_end(&self) -> bool {
        return self.get_char(self.current) == '\0';
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        return Token {
            message: String::from(""),
            ttype: ttype,
            start: self.start,
            length: (self.current - self.start),
            line: self.line,
        };
    }

    fn error_token(&self, message: String) -> Token {
        return Token {
            message: message.clone(),
            ttype: TokenType::Error,
            start: 0,
            length: message.len(),
            line: self.line,
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.get_char(self.current - 1);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_char(self.current) != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        return self.get_char(self.current);
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.get_char(self.current);
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token(String::from("Unterminated string."));
        }
        self.advance();
        return self.make_token(TokenType::String);
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        return self.make_token(TokenType::Number);
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && self.source[self.start + start..self.start + start + length] == *rest
        {
            return token_type;
        }

        TokenType::Identifier
    }

    fn identifier_type(&self) -> TokenType {
        match self.get_char(self.start) {
            'a' => return self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => return self.check_keyword(1, 1, "f", TokenType::If),
            'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
            'f' => {
                if self.current - self.start > 1 {
                    match self.get_char(self.start + 1) {
                        'a' => return self.check_keyword(1, 2, "lse", TokenType::False),
                        'o' => return self.check_keyword(1, 4, "r", TokenType::For),
                        'u' => return self.check_keyword(1, 3, "n", TokenType::Fun),
                        _ => panic!("Unexpected character"),
                    }
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.get_char(self.start + 1) {
                        'h' => return self.check_keyword(1, 2, "lse", TokenType::This),
                        'r' => return self.check_keyword(1, 4, "r", TokenType::True),
                        _ => panic!("Unexpected character"),
                    }
                }
            }
            'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            _ => panic!("Unexpected character"), // or handle the default case appropriately
        }
        return TokenType::Identifier;
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if c.is_alphabetic() {
            return self.identifier();
        }
        if c.is_digit(10) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let token: TokenType = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token)
            }
            '=' => {
                let token: TokenType = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token)
            }
            '<' => {
                let token: TokenType = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token)
            }
            '>' => {
                let token: TokenType = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token)
            }
            '"' => self.string(),
            _ => panic!("Unexpected character"), // or handle the default case appropriately
        };

        return self.error_token(String::from("Unexpected character."));
    }
}
