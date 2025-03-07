use ast::Statement;
use lexer::{Lexer, Token};
use parser::Parser;
use visitor_sexpression::ToSExpression;

mod ast;
mod lexer;
mod parser;
mod visitor_sexpression;

fn main() {
    let mut lexer = Lexer::new(r#"12 17 "Hello"   'world' "#);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);

    let mut parser = Parser::new("12;");
    let ast = parser.parse();
    let sexpression = ast.to_sexpression().unwrap();
    println!("SExpression: {}", sexpression);
}
