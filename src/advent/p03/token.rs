type Num = i32;

#[derive(Debug)]
pub enum Token {
    Mul(Num, Num),
    DO,
    DONT,
}

pub struct Tokenizer {
    content: Vec<char>,
    current: usize,
}

macro_rules! match_token {
    ($self: expr, $consumer: expr) => {{
        let start = $self.current;
        if let Some(res) = $consumer {
            return Some(res);
        }
        $self.current = start;
    }};
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect(),
            current: 0,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        while !self.is_end() {
            match_token!(self, self.mul());
            match_token!(self, self.dont());
            match_token!(self, self.match_do());
            self.advance();
        }
        None
    }

    fn is_end(&self) -> bool {
        self.current >= self.content.len()
    }

    fn curr(&self) -> char {
        match self.content.get(self.current) {
            Some(c) => *c,
            None => '\0',
        }
    }

    fn peek(&self, offset: usize) -> char {
        match self.content.get(self.current + offset) {
            Some(c) => *c,
            None => '\0',
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn mul(&mut self) -> Option<Token> {
        self.consume_str("mul(")?;
        let left = self.number()?;
        self.consume(',')?;
        let right = self.number()?;
        self.consume(')')?;
        Some(Token::Mul(left, right))
    }

    fn dont(&mut self) -> Option<Token> {
        self.consume_str("don't()")?;
        Some(Token::DONT)
    }

    fn match_do(&mut self) -> Option<Token> {
        self.consume_str("do()")?;
        Some(Token::DO)
    }

    fn consume_str(&mut self, s: &str) -> Option<()> {
        for (i, c) in s.chars().enumerate() {
            if self.peek(i) != c {
                return None;
            }
        }
        self.current += s.len();
        Some(())
    }

    fn consume(&mut self, c: char) -> Option<()> {
        if self.curr() == c {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn slice(&self, start: usize, end: usize) -> String {
        self.content.as_slice()[start..end].iter().collect()
    }

    fn number(&mut self) -> Option<Num> {
        const MAX_LENGTH: usize = 3;
        let mut i = 0;
        while i < MAX_LENGTH && self.peek(i).is_numeric() {
            i += 1;
        }
        let num = self.slice(self.current, self.current + i);
        let num = num.parse().ok()?;
        self.current += i;
        Some(num)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        Tokenizer::next(self)
    }
}
