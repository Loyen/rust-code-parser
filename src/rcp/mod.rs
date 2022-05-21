mod lexer;
mod parser;

fn compile(content: &String) -> String {
    let tokens = lexer::tokenize(&content);

    let output = parser::parse_tokens(tokens);
    return output;
}

pub fn run(content: String) -> String {
    return compile(&content);
}
