use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    macros: HashMap<String, Vec<Vec<Word>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
enum Word {
    Add,
    Subtract,
    Multiply,
    Divide,
    DUP,
    DROP,
    SWAP,
    OVER,
    Number(Value),
    Macro(String, usize),
}

impl Forth {
    pub fn new() -> Forth {
        Self { stack: vec![], macros: HashMap::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    // split and send to tokenizer
    pub fn eval(&mut self, input: &str) -> Result {
        let mut iter = input.split_whitespace();
        while let Some(token) = iter.next() {
            self.parse_token(token, &mut iter)?;
        }
        Ok(())
    }

    // process macro definitions separately and parse known words one by one
    fn parse_token<'a>(&mut self, token: &str, rest: &mut impl Iterator<Item = &'a str>) -> Result {
        match token {
            ":" => self.parse_definition(rest)?,
            _ => {
                let word = self.parse_word(token)?;
                self.eval_word(word)?
            }
        }
        Ok(())
    }


    fn parse_definition<'a>(&mut self, rest: &mut impl Iterator<Item=&'a str>) -> Result {
        let mut words = vec![];
        if let Some(name) = rest.next() && name.parse::<Value>().is_err() {
            while let Some(token) = rest.next() && token != ";" {
                words.push(self.parse_word(token)?);
            }

            // each time new version is stored
            self.macros.entry(name.to_ascii_uppercase()).or_default().push(words);
            Ok(())
        }
        else { Err(Error::InvalidWord) }

    }

    // finds existing macro or known word
    fn parse_word(&self, word: &str) -> std::result::Result<Word, Error> {
        if let Some(value) = self.get_last_macro_version(word.to_ascii_uppercase()) {
            return Ok(Word::Macro(word.to_ascii_uppercase(), value));
        }
        Self::parse_known_word(word)
    }

    fn parse_known_word(word: &str) -> std::result::Result<Word, Error> {
        let result = match word.to_ascii_uppercase().as_str() {
            "+" => Word::Add,
            "-" => Word::Subtract,
            "/" => Word::Divide,
            "*" => Word::Multiply,
            "DUP" => Word::DUP,
            "DROP" => Word::DROP,
            "SWAP" => Word::SWAP,
            "OVER" => Word::OVER,
            _ => {
                if let Ok(n) = word.parse::<Value>() {
                    Word::Number(n)
                } else { return Err(Error::UnknownWord) }
            }
        };
        Ok(result)
    }

    fn get_last_macro_version(&self, macro_name: String) -> Option<usize> {
        self.macros.get(&macro_name).map(|v| v.len() - 1)
    }

    fn expand_macro(&self, macro_name: String, version: usize) -> Option<Vec<Word>> {
        self.macros.get(&macro_name)
            .and_then(|v| v.get(version))
            .cloned()
    }

    // evaluates standard words
    fn eval_word(&mut self, word: Word) -> Result {
        match word {
            Word::Add => self.eval_stack_binary_operation(|a, b| a + b)?,
            Word::Subtract => self.eval_stack_binary_operation(|a, b| b - a)?,
            Word::Multiply =>  self.eval_stack_binary_operation(|a, b| a * b)?,
            Word::Divide => {
                self.stack_has_at_least(2)?;
                let (a,b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                if a == 0 { return Err(Error::DivisionByZero) }
                self.stack.push(b / a);
            }
            Word::DUP => {
                self.stack_has_at_least(1)?;
                self.stack.push(self.stack[self.stack.len() - 1]);
            },
            Word::DROP => {
                self.stack_has_at_least(1)?;
                self.stack.pop();
            },
            Word::SWAP => {
                self.stack_has_at_least(2)?;
                let last = self.stack.len() - 1;
                let prev = last - 1;
                (self.stack[last], self.stack[prev]) = (self.stack[prev], self.stack[last]);

            },
            Word::OVER => {
                self.stack_has_at_least(2)?;
                self.stack.push(self.stack[self.stack.len() - 2]);
            },
            Word::Number(n) => self.stack.push(n),
            Word::Macro(name, version) => self.eval_macro(name, version)?,
        }
        Ok(())
    }

    fn eval_stack_binary_operation(&mut self, op: fn(a: Value, b: Value) -> Value) -> Result {
        self.stack_has_at_least(2)?;
        let a = self.stack.pop().unwrap();
        let b  = self.stack.pop().unwrap();
        self.stack.push(op(a,b));
        Ok(())
    }

    fn stack_has_at_least(&self, value: usize) -> Result {
        if self.stack.len() < value {
            return Err(Error::StackUnderflow);
        }
        Ok(())
    }

    fn eval_macro(&mut self, macro_name: String, version: usize) -> Result {
        for word in self.expand_macro(macro_name, version).unwrap() {
            self.eval_word(word)?;
        }
        Ok(())
    }
}
