#[derive(Debug)]
pub enum Tt {
    EQUAL, PLUS, MINUS, MULT, DIV, MOD,
    IF, ELSE, FUN, LET, CONST, WHILE, FOR, RETURN, CONTINUE, BREAK, CLASS, EXTEND, NIL, IN, AND, OR,
    NUMBER, STRING, IDENTIFIER, EOF, ERROR,
}

pub struct Scanner {
    start: usize,
    current: usize,
    column: usize,
    line: usize,
    src: Vec<char>
}

impl Scanner {
    pub fn new(src: String)->Self {
        Self {
            start: 0,
            current: 0,
            column: 0,
            line: 1,
            src: src.chars().collect()
        }
    }

    fn eof(&self)->bool {
        self.current >= self.src.len()
    }

    fn peek(&self) -> char {
        *self.src.get(self.current).expect("fail to read char")
    }

    fn next(&mut self) -> char {
        let c = self.src.get(self.current).expect("fail to read char");
        if *c == '\n' {
            self.line += 1;
            self.column = 0
        } else {
            self.column += 1;
        }
        self.current += 1;
        *c
    }

    fn make_token(&self, kind: Tt) -> Token {
        Token {
            kind: kind,
            start: self.start,
            end: self.current,
            line: self.line,
            column: self.column
        }
    }

    fn skip_space(&mut self) {
        while !self.eof() && (self.peek() == ' ' || self.peek() == '\t') {
            self.next();
        }
    }

    fn number(&mut self) -> Token {
        while !self.eof() && self.peek().is_ascii_digit() {
            self.next();
        }
        if !self.eof() && self.peek() == '.' {
            self.next();
            while !self.eof() && self.peek().is_ascii_digit() {
                self.next();
            }
        }
        self.make_token(Tt::NUMBER)
    }

    fn string(&mut self) -> Token {
        if self.eof() {
            self.make_token(Tt::EOF)
        } else {
            while !self.eof() && self.peek() != '"' {
                self.next();
            }
            if self.eof() {
                self.make_token(Tt::EOF)
            } else {
                self.next();
                self.make_token(Tt::STRING)
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while !self.eof() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            self.next();
        }
        let start = self.src.get(self.start).expect("cant read char again");
        return match start {
            'i' => {
                if self.current - self.start == 2 {
                    let second = self.src.get(self.start + 1).expect("cant read char second");
                    return match second {
                        'f' => self.make_token(Tt::IF),
                        'n' => self.make_token(Tt::IN),
                        _ => self.make_token(Tt::IDENTIFIER)
                    }
                } else {
                    self.make_token(Tt::IDENTIFIER)
                }
            },
            'c' => {
                let second = self.src.get(self.start + 1).unwrap();
                match second {
                    'o' => {
                        let third = self.src.get(self.start + 2);
                        match third {
                            'n' => {
                            },
                            _ => self.make_token(Tt::IDENTIFIER)
                        }
                    }
                }
            },
            _ => self.make_token(Tt::IDENTIFIER)
        };
        self.make_token(Tt::IDENTIFIER)
    }

    pub fn next_token(&mut self)->Token {
        self.skip_space();
        if self.eof() {
            return self.make_token(Tt::EOF);
        }

        self.start = self.current;
        let c = self.next();
        if c.is_ascii_digit() {
            self.number()
        } else if c == '"' {
            self.string()
        } else if c.is_ascii_alphabetic() || c == '_' {
            self.identifier()
        } else {
            self.make_token(Tt::ERROR)
        }
    }
}

#[derive(Debug)]
pub struct Token {
    kind: Tt,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
}
