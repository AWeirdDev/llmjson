#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Token {
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    Quote,
    Backslash,
    Comma,
    Colon,
    Atom(char),
    Atomic(String),
    Eof
}

impl Token {
    pub fn new(c: char) -> Self {
        match c {
            '{' => Self::LCurly,
            '}' => Self::RCurly,
            '[' => Self::LBracket,
            ']' => Self::RBracket,
            '"' => Self::Quote,
            '\\' => Self::Backslash,
            ',' => Self::Comma,
            ':' => Self::Colon,
            _ => Self::Atom(c)
        }
    }
}

trait TokenControl<T> {
    fn merge(&mut self) -> &mut Self;
}

impl TokenControl<Token> for Vec<Token> {
    fn merge(&mut self) -> &mut Vec<Token> {
        let mut i = 0;
        let mut atomic = String::new();
        loop {
            let item = &self[i];
            match item {
                Token::Eof => break,
                Token::Atom(c) => {
                    atomic.push(*c);
                    self.remove(i);
                    continue;
                }
                _ => {
                    if !atomic.is_empty() {
                        self.insert(i, Token::Atomic(atomic.to_owned()));
                        atomic.clear();
                    }
                }
            }
            i += 1;
        }

        self
    }
}

pub trait Append<T> {
    fn append(self, value: T) -> Self;
}

impl<T> Append<T> for Vec<T> {
    fn append(mut self, value: T) -> Self {
        self.push(value);
        self
    }
}

impl<T> Append<&[T]> for Vec<T>
where
    T: Clone
{
    fn append(mut self, value: &[T]) -> Self {
        self.extend_from_slice(value);
        self
    }
}

fn main() {
    let code = r#"{
        "name": "walter",
        "age": 55.5,
        oops: "i missed the quotation marks!"
    }"#;
    let mut tokens = code.chars().map(|c| Token::new(c)).collect::<Vec<_>>().append(Token::Eof);
    println!("{:?}", tokens.merge());
}
