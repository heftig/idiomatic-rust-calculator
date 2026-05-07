pub struct Evaluator<'a> {
    input: &'a [u8],
}

impl<'a> Evaluator<'a> {
    pub fn eval(input: &'a [u8]) -> i32 {
        let mut evaluator = Self { input };
        evaluator.parse_expr()
    }

    fn peek_char(&self) -> Option<u8> {
        self.input.first().copied()
    }

    fn advance(&mut self) {
        self.input = self.input.get(1..).unwrap_or(&[]);
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn parse_number(&mut self) -> i32 {
        self.skip_ws();
        let mut res = 0;
        while let Some(c) = self.peek_char() {
            if c >= b'0' && c <= b'9' {
                res = res * 10 + (c - b'0') as i32;
                self.advance();
            } else {
                break;
            }
        }
        res
    }
    fn parse_term(&mut self) -> i32 {
        let mut res = self.parse_number();
        loop {
            self.skip_ws();
            match self.peek_char() {
                Some(b'*') => {
                    self.advance();
                    res *= self.parse_number();
                }
                Some(b'/') => {
                    self.advance();
                    let divisor = self.parse_number();
                    if divisor != 0 {
                        res /= divisor;
                    }
                }
                _ => break,
            }
        }
        res
    }
    fn parse_expr(&mut self) -> i32 {
        let mut res = self.parse_term();
        loop {
            self.skip_ws();
            match self.peek_char() {
                Some(b'+') => {
                    self.advance();
                    res += self.parse_term();
                }
                Some(b'-') => {
                    self.advance();
                    res -= self.parse_term();
                }
                _ => break,
            }
        }
        res
    }
}
