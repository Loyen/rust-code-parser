use rcp::lexer::Token;
use rcp::lexer::TokenTypes;

pub fn parse_tokens(tokens: Vec<Token>) -> String {
    let mut parser = Parser {
        tokens: tokens,
    };

    return parser.parse_tokens();
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn parse_tokens(&mut self) -> String {
        let mut output: String = "".to_string();

        while self.tokens.len() > 0 {
            let token: Token = self.shift_token();

            let statement_output = match token.code {
                TokenTypes::TPrint => self.statement_print(),
                _ => panic!("Unhandled token while parsing statements"),
            };

            output.push_str(&statement_output);
        }

        return output;
    }

    fn shift_token(&mut self) -> Token {
        if self.tokens.len() == 0 {
            panic!("Unexpected end of file");
        }

        let token: Token = self.tokens[0].to_owned();

        self.tokens.remove(0);

        return token;
    }

    fn statement_print(&mut self) -> String {
        return self.parse_expression();
    }

    fn parse_expression(&mut self) -> String {
        let mut output: String = "".to_string();

        while self.tokens.len() > 0 {
            let token: Token = self.shift_token();

            let statement_output = match token.code {
                TokenTypes::TString => token.value,
                _ => panic!("Unhandled token while parsing expression"),
            };

            output.push_str(&statement_output);
        }

        return output;
    }
}
