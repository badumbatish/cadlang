use cadlang::frontend::lexer::Token;
use logos::Logos;

fn main() {
    println!("Hello, world!");
    for result in Token::lexer("Create ridiculously fast Lexers.") {
        match result {
            Ok(token) => println!("{:?}", token),
            Err(e) => panic!("some error occurred: {:?}", e),
        }
    }
}
