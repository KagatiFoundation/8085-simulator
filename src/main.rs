
#[derive(Clone, Debug)]
enum Opcode {
    Add(Register),
    Adc(Register),
    Sub(Register),
    Sbb(Register),
    Ana(Register),
    Xra(Register),
    Ora(Register),
    Cmp(Register),
    Mov(Register, Register),
    Mvi(Register, u8),
    Lxi(RegisterPair, u16),
    Sta(u16),
    Lda(u16),
    Shld(u16),
    Lhld(u16),
    Stax(RegisterPair),
    Ldax(RegisterPair),
    Xchg,
    Xthl,
    Sphl,
    Pchl,
    Jmp(u16),
    Jnz(u16),
    Jz(u16),
    Jnc(u16),
    Jc(u16),
    Jpo(u16),
    Jpe(u16),
    Jp(u16),
    Jm(u16),
    Call(u16),
    Cnz(u16),
    Cz(u16),
    Cnc(u16),
    Cc(u16),
    Cpo(u16),
    Cpe(u16),
    Cp(u16),
    Cm(u16),
    Ret,
    Rnz,
    Rz,
    Rnc,
    Rc,
    Rpo,
    Rpe,
    Rp,
    Rm,
    Rst(u8),
    Dcr(Register),
    Inr(Register),
    Dcx(RegisterPair),
    Inx(RegisterPair),
    Cma,
    Cmc,
    Stc,
    Dad(RegisterPair),
    Push(RegisterPair),
    Pop(RegisterPair),
    Hlt,
    Eof,
}

#[derive(Clone, Debug)]
enum Operand {
    Register(Register),
    RegisterPair(RegisterPair),
    Immediate(u8),
    Address(u16),
}

#[derive(Clone, Debug)]
struct Token<'a> {
    opcode: Opcode,
    column: usize,
    lexeme: &'a str,
    line: u32,
}

#[derive(Clone, Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    None
}

impl Register {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'H' => Self::H,
            'L' => Self::L,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug)]
enum RegisterPair {
    B,
    D,
    H,
    S,
}

#[derive(Debug)]
struct Scanner<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
    start: usize,
    source: &'a str,
    line: u32,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            tokens: vec![],
            current: 0,
            start: 0,
            source,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            opcode: Opcode::Eof,
            column: 0,
            lexeme: "",
            line: self.line
        });
        self.tokens.to_vec()
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

     
    fn consume_whitespace(&mut self) {
        self.advance();
        while self.peek_next().is_whitespace()  {
            self.advance();
        }
    }

    fn op_code(&mut self) {
        let registers = vec!['A', 'B', 'C', 'D', 'E', 'H', 'L'];
        while self.peek().is_alphabetic() {
            self.advance();
        }

        if let Some(text) = self.source.get(self.start..self.current + 1) {
            match text.trim() {
                // MOV A, B
                "MOV" => {
                    if self.peek_next().is_ascii_alphabetic() {
                        self.advance();
                        let operator1 = Register::from_char(self.peek());
                        if self.peek_next().is_whitespace()  {
                            self.consume_whitespace()
                        }
                        if self.peek_next() == ',' {
                            self.consume_whitespace();
                            let operator2 = Register::from_char(self.peek_next());
                            self.add_token(Opcode::Mov(operator1, operator2));
                        }
                    }
                }, 
                // MVI A, 0x00
                "MVI" => {
                    if self.peek_next().is_ascii_alphabetic() {
                        self.advance();
                        let operator1 = Register::from_char(self.peek());
                        if self.peek_next().is_whitespace()  {
                            self.consume_whitespace()
                        }
                        if self.peek_next() == ',' {
                            self.consume_whitespace();
                            let operator2 = &self.source[self.current..self.current + 3];
                            println!("operator 2 is {}",operator2);
                            self.advance();
                            // self.add_token(Opcode::Mvi(operator1, operator));
                        }
                    }
                },
                _ => {}
            }
        }
    }


    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ('a'..='z') | ('A'..='Z') => self.op_code(),
            _ => {} // 'M' => {
                    //     print!("len is {}", self.source.len());
                    //     if self.peek() == 'O' && self.peek_next() == 'V' {
                    //         let current = self.current;

                    //         let mut end = current;
                    //         while self.peek() != '\n' || self.peek() != '\0' {
                    //             self.advance();

                    //             end += 1;
                    //         }

                    //         let operands = self.source[current + 1..end]
                    //             .split(",")
                    //             .collect::<Vec<&str>>();
                    //         let operand1 = Register::from_char(operands[0].trim().chars().nth(0).unwrap());
                    //         let operand2 = Register::from_char(operands[1].trim().chars().nth(0).unwrap());
                    //         self.add_token(Opcode::Mov(operand1, operand2), None)
                    //     }
                    // }
        }
    }

    fn add_token(&mut self, opcode: Opcode) {
        let text = &self.source[self.start..self.current];
        if self.peek() == '\n' {
            self.line += 1;
        }
        self.tokens.push(Token {
            opcode,
            column: self.current,
            lexeme: text,
            line: self.line
        });
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn main() {
    let instructions = r#"
    MOV B, C
    MVI A, 03
    "#;
    let mut scanner = Scanner::new(instructions);
    scanner.scan_tokens();
    println!("{:#?}", scanner);
}
