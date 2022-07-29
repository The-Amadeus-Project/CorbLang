mod lexer;

fn main() {
    println!("{:?}", lexer::Lexer::new().lex_text("".to_string()));
}
