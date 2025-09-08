use std::{collections::HashMap, fmt};
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl Error for ParseError {}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        '\u{08}' => write!(f, "\\b")?,
                        '\u{0C}' => write!(f, "\\f")?,
                        _ => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            JsonValue::Array(a) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(o) => {
                write!(f, "{{")?;
                for (i, (key, value)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

pub struct Parser {
    input: Vec<char>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    fn consume_str(&mut self, s: &str) -> Result<(), ParseError> {
        for expected_char in s.chars() {
            match self.next_char() {
                Some(c) if c == expected_char => continue,
                Some(c) => return Err(self.error(&format!("Expected '{}', found '{}'", expected_char, c))),
                None => return Err(self.error(&format!("Expected '{}', found end of input", expected_char))),
            }
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            position: self.position,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        self.skip_whitespace();
        let result = self.parse_value()?;
        self.skip_whitespace();
        if self.peek_char().is_some() {
            return Err(self.error("unexpected trailing characters"));
        }
        Ok(result)
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        self.skip_whitespace();
        let c = self.peek_char().ok_or_else(|| self.error("unexpected end of input"))?;
        match c {
            'n' => self.parse_null(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            '"' => self.parse_string(),
            '0'..='9' | '-' => self.parse_number(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            _ => Err(self.error(&format!("unexpected character: {}", c))),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, ParseError> {
        self.consume_str("null")?;
        Ok(JsonValue::Null)
    }

    fn parse_true(&mut self) -> Result<JsonValue, ParseError> {
        self.consume_str("true")?;
        Ok(JsonValue::Boolean(true))
    }

    fn parse_false(&mut self) -> Result<JsonValue, ParseError> {
        self.consume_str("false")?;
        Ok(JsonValue::Boolean(false))
    }

    fn parse_string(&mut self) -> Result<JsonValue, ParseError> {
        self.next_char();
        let mut result = String::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => return Ok(JsonValue::String(result)),
                '\\' => {
                    let escaped_char = self.next_char()
                        .ok_or_else(|| self.error("unterminated escape sequence"))?;
                    match escaped_char {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\u{0008}'), 
                        'f' => result.push('\u{000C}'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        _ => return Err(self.error(&format!("invalid escape sequence: \\{}", escaped_char))),
                    }
                }
                _ => result.push(c),
            }
        }
        Err(self.error("Unterminated string"))
    }

    fn parse_number(&mut self) -> Result<JsonValue, ParseError> {
        let start_pos = self.position;
        let mut number_str = String::new();

        if let Some('-') = self.peek_char() {
            number_str.push(self.next_char().unwrap());
        }
        
        
        match self.peek_char() {
            Some('0') => {
                number_str.push(self.next_char().unwrap());
            }
            Some(c) if c.is_ascii_digit() => {
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        number_str.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
            }
            _ => return Err(self.error("expected digit after minus sign or invalid number")),
        }
        
        if let Some('.') = self.peek_char() {
            number_str.push(self.next_char().unwrap()); // consume '.'
            
            let mut has_decimal_digits = false;
            while let Some(c) = self.peek_char() {
                if c.is_ascii_digit() {
                    number_str.push(self.next_char().unwrap());
                    has_decimal_digits = true;
                } else {
                    break;
                }
            }
            
            if !has_decimal_digits {
                return Err(self.error("expected digit after decimal point"));
            }
        }
        
        if let Some(c) = self.peek_char() {
            if c == 'e' || c == 'E' {
                number_str.push(self.next_char().unwrap()); // consume 'e' or 'E'
                
                if let Some(sign) = self.peek_char() {
                    if sign == '+' || sign == '-' {
                        number_str.push(self.next_char().unwrap());
                    }
                }
                
                let mut has_exp_digits = false;
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        number_str.push(self.next_char().unwrap());
                        has_exp_digits = true;
                    } else {
                        break;
                    }
                }
                
                if !has_exp_digits {
                    return Err(self.error("expected digit in exponent"));
                }
            }
        }
        
        match number_str.parse::<f64>() {
            Ok(num) => Ok(JsonValue::Number(num)),
            Err(_) => Err(ParseError {
                message: format!("invalid number format: '{}'", number_str),
                position: start_pos,
            }),
        }


    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        self.next_char();
        self.skip_whitespace();

        let mut elements = Vec::new();

        if let Some(']') = self.peek_char() {
            self.next_char();
            return Ok(JsonValue::Array(elements));
        }

        loop {
            let value = self.parse_value()?;
            elements.push(value);

            self.skip_whitespace();

            match self.peek_char() {
                Some(',') => {
                    self.next_char();
                    self.skip_whitespace();

                    if let Some(']') = self.peek_char() {
                        return Err(self.error("unexptected trailing comma in array"));

                    }
                }
                Some(']') => {
                    self.next_char();
                    break;
                }
                Some(c) => return Err(self.error(&format!("expected ',' or ']' in array, found '{}'", c))),
                None => return Err(self.error("unterminated array")),
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        self.next_char();
        self.skip_whitespace();

        let mut object = HashMap::new();

        if let Some('}') = self.peek_char() {
            self.next_char();
            return Ok(JsonValue::Object(object));
        }

        loop {
            self.skip_whitespace();
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err(self.error("object keys must be strings")),
            };

            self.skip_whitespace();
            match self.next_char() {
                Some(':') => {},
                Some(c) => return Err(self.error(&format!("expected ':' after object key, found '{}'", c))),
                None => return Err(self.error("expected ':' after object key, found end of input")),

            }

            self.skip_whitespace();
            let value = self.parse_value()?;

            object.insert(key, value);

            self.skip_whitespace();

            match self.peek_char() {
                Some(',') => {
                    self.next_char();
                    self.skip_whitespace();

                    if let Some('}') = self.peek_char() {
                        return Err(self.error("unexpoected trailing comma in object"));
                    }
                }
                Some('}') => {
                    self.next_char();
                    break;
                }
                Some(c) => return Err(self.error(&format!("expected ',' oor '}}' in object, found '{}'", c))),
                None => return Err(self.error("unterminated object")),

            }
        }

        Ok(JsonValue::Object(object))
    }
}

