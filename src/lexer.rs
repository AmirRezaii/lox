use core::fmt;

#[derive(Debug,Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.text.is_empty() {
            write!(f, "(Token: {})", self.kind)
        } else {
            write!(f, "(Token: {}, Value: {})", self.kind, self.text)
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, text: &str, line: u32) -> Self {
        Self { kind: kind, text: text.to_string(), line: line }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Slash,
    Star,

    Equal,
    Greater,
    Less,
    EqualEqual,
    NotEqual,
    GreaterEq,
    LessEq,

    CloseParen,
    OpenParen,
    CloseBrace,
    OpenBrace,
    SemiColon,
    Dot,
    Comma,
    Bang,


    Number,
    Identifier,
    StringLiteral,

    Puts,
    Let,
    For,
    While,
    Fun,
    If,
    Else,
    Return,
    Nil,
    Or,
    And,
    True,
    False,

    WhiteSpace,
    NewLine,
    Comment,
    Invalid,
} 

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match *self {
            Plus => write!(f, "Plus"),
            Minus => write!(f, "Minus"),
            Slash => write!(f, "Slash"),
            Star => write!(f, "Star"),

            Equal => write!(f, "Equal"),
            Greater => write!(f, "Greater"),
            Less => write!(f, "Less"),

            CloseParen => write!(f, "CloseParen"),
            OpenParen => write!(f, "OpenParen"),
            CloseBrace => write!(f, "CloseBrace"),
            OpenBrace => write!(f, "OpenBrace"),
            SemiColon => write!(f, "SemiColon"),
            Dot => write!(f, "Dot"),
            Comma => write!(f, "Comma"),
            Bang => write!(f, "Bang"),

            EqualEqual => write!(f, "EqualEqual"),
            NotEqual => write!(f, "NotEqual"),
            GreaterEq => write!(f, "GreaterEq"),
            LessEq => write!(f, "LessEq"),

            Number => write!(f, "Number"),
            Identifier => write!(f, "Identifier"),
            StringLiteral => write!(f, "String"),

            Puts => write!(f, "Puts"),
            Let => write!(f, "Let"),
            For => write!(f, "For"),
            While => write!(f, "While"),
            Fun => write!(f, "Fun"),
            If => write!(f, "If"),
            Else => write!(f, "Else"),
            Return => write!(f, "Return"),
            Nil => write!(f, "Nil"),
            Or => write!(f, "Or"),
            And => write!(f, "And"),
            True => write!(f, "True"),
            False => write!(f, "False"),

            WhiteSpace => write!(f, "WhiteSpace"),
            NewLine => write!(f, "NewLine"),
            Comment => write!(f, "Comment"),
            Invalid => write!(f, "Invalid"),
        }
    }
}

pub fn trim_tokens(tokens: Vec<Token>) -> Vec<Token> {
    tokens.into_iter()
        .filter(|t| t.kind != TokenKind::WhiteSpace && t.kind != TokenKind::NewLine)
        .collect()
}
pub fn full_trim_tokens(tokens: Vec<Token>) -> Vec<Token> {
    tokens.into_iter()
        .filter(|t| t.kind != TokenKind::WhiteSpace && t.kind != TokenKind::NewLine && t.kind != TokenKind::Comment)
        .collect()
}
pub fn is_keyword(s: &String) -> Option<TokenKind> {
    match s.as_str() {
        "puts" => Some(TokenKind::Puts),
        "let" => Some(TokenKind::Let),
        "for" => Some(TokenKind::For),
        "while" => Some(TokenKind::While),
        "fun" => Some(TokenKind::Fun),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "return" => Some(TokenKind::Return),
        "Nil" => Some(TokenKind::Nil),
        "or" => Some(TokenKind::Or),
        "and" => Some(TokenKind::And),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        _ => None,
    }
}

pub fn scanner(input: String) -> Vec<Token> {
    let mut char_indices = input.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;

    while let Some((pos, ch)) = char_indices.next() {
        use TokenKind::*;
        let token = match ch {
            '+' => Token::new(Plus, "", line),
            '-' => Token::new(Minus, "", line),
            '/' => {
                match char_indices.next_if_eq(&(pos+1, '/')) {
                    Some(_slash) => {
                        let mut last_matched: char = '\0';
                        let s: String = char_indices
                            .by_ref()
                            .take_while(|(_pos, c)| {
                                last_matched = *c;
                                *c != '\n'
                            })
                            .map(|(_pos, c)| { c })
                            .collect();

                        match last_matched {
                            '\n' => Token::new(Comment, s.as_str(), line),
                            _ => Token::new(Invalid, "Invalid comment.", line),
                        }
                    },
                    None => Token::new(Slash, "", line)
                }
            },
            '*' => Token::new(Star, "", line),

            '=' => {
                match char_indices.next_if_eq(&(pos+1, '=')) {
                    Some(_equals) => Token::new(EqualEqual, "", line),
                    None => Token::new(Equal, "", line),
                }
            },
            '>' => {
                match char_indices.next_if_eq(&(pos+1, '=')) {
                    Some(_equals) => Token::new(GreaterEq, "", line),
                    None => Token::new(Greater, "", line),
                }
            }
            '<' => {
                match char_indices.next_if_eq(&(pos+1, '=')) {
                    Some(_equals) => Token::new(LessEq, "", line),
                    None => Token::new(Less, "", line),
                }
            }
            '!' => {
                match char_indices.next_if_eq(&(pos+1, '=')) {
                    Some(_equals) => Token::new(NotEqual, "", line),
                    None => Token::new(Bang, "", line),
                }
            },
            '"' => {
                let mut last_matched: char = '\0';
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        last_matched = *c;
                        *c != '"'
                    })
                    .map(|(_pos, c)| { c })
                    .collect();

                match last_matched {
                    '"' => Token::new(StringLiteral, s.as_str(), line),
                    _ => Token::new(Invalid, "Invalid literal.", line),
                }
            },
            '(' => Token::new(OpenParen, "", line),
            ')' => Token::new(CloseParen, "", line),
            '{' => Token::new(OpenBrace, "", line),
            '}' => Token::new(CloseBrace, "", line),

            ';' => Token::new(SemiColon, "", line),
            '.' => Token::new(Dot, "", line),
            ',' => Token::new(Comma, "", line),

            ' ' => Token::new(WhiteSpace, "", line),
            '\n' => {
                line += 1;
                Token::new(NewLine, "", line-1)
            },

            c if char::is_numeric(c) => {
                let mut s = String::new();
                while let Some((_pos, c)) = char_indices
                    .by_ref()
                    .next_if(|(_pos, ch)| char::is_alphanumeric(*ch))
                    {
                        s.push(c);
                    }

                s.insert(0, c);
                if let Ok(num) = s.parse::<i32>() {
                    Token::new(Number, s.as_str(), line)
                } else {
                    Token::new(Invalid, "Not a valid number.", line)
                }
            },
            c if char::is_alphabetic(c) => {
                let mut s = String::new();
                while let Some((_pos, c)) = char_indices
                    .by_ref()
                    .next_if(|(_pos, ch)| char::is_alphanumeric(*ch))
                    {
                        s.push(c);
                    }

                s.insert(0, c);
                if let Some(t) = is_keyword(&s) {
                    Token::new(t, "", line)
                } else {
                    Token::new(Identifier, s.as_str(), line)
                }
            },
            _ => Token::new(Invalid, format!("{}", ch).as_str(), line),
        };
        tokens.push(token);
    }

    tokens
}
