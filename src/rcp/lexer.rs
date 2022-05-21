#[derive(Clone)]
pub enum TokenTypes {
    TString,
    TPrint,
}

#[derive(Clone)]
pub struct Token {
    pub line: u32,
    pub code: TokenTypes,
    pub value: String,
}

enum Modes {
    MNormal,
    MString,
}

pub fn tokenize(content: &String) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        line_number: 1,
        current_mode: Modes::MNormal,
        token_list: Vec::new(),
        buffer: "".to_string(),
    };

    return tokenizer.tokenize(&content);
}

struct Tokenizer {
    line_number: u32,
    current_mode: Modes,
    token_list: Vec<Token>,
    buffer: String,
}

impl Tokenizer {
    fn tokenize(&mut self, content: &String) -> Vec<Token> {
        self.token_list = Vec::new();
        self.line_number = 1;
        self.current_mode = Modes::MNormal;
        self.buffer = "".to_string();

        for character in content.chars() {
            if character.to_string().eq("\n") {
                self.line_number = self.line_number + 1;
            }

            match self.current_mode {
                Modes::MNormal => self.mode_normal_parse_char(&character),
                Modes::MString => self.mode_string_parse_char(&character),
            };
        }

        return self.token_list.to_owned()
    }

    fn mode_normal_parse_char(&mut self, character: &char) {
        if self.is_whitespace(character) {
            return;
        }
        self.buffer.push(*character);

        if self.buffer.eq("\"") {
            self.current_mode = Modes::MString;
            self.buffer = "".to_string();
        } else if self.buffer.eq("print") {
            self.token_list.push(Token {
                line: self.line_number,
                code: TokenTypes::TPrint,
                value: "".to_string()
            });
            self.buffer = "".to_string();
        }
    }

    fn mode_string_parse_char(&mut self, character: &char) {
        if character.to_string().eq("\"") {
            self.token_list.push(Token {
                line: self.line_number,
                code: TokenTypes::TString,
                value: self.buffer.to_string()
            });
            self.buffer = "".to_string();

            self.current_mode = Modes::MNormal;
        } else {
            self.buffer.push(*character);
        }
    }

    fn is_whitespace(&self, character: &char) -> bool {
        let character_str: String = character.to_string();

        if character_str.eq(" ") {
            return true;
        } else if character_str.eq("\n") {
            return true;
        }

        return false;
    }
}
