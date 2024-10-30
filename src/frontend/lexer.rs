use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    /// Decl of a variable
    #[token("let", |lex| lex.slice().to_owned())]
    Let(String),

    /// A floating point number, this is from Logos (https://logos.maciej.codes/examples/json.html?highlight=number#json-parser)
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[regex(r"([_a-zA-Z][_a-zA-Z0-9]*)", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[token(".")]
    Dot,

    #[token("cell")]
    Cell,

    #[token(";")]
    Semicomma,
}

impl Token {}
//pub fn lex(input_string: &String) -> impl Iterator<Item = (Token, Simple)> {
//    let token_iter = Token::lexer(input_string)
//        .spanned()
//        .map(|(token, token_span)| match token {
//            Ok(token) => (token, token_span.into()),
//            Err(()) => (Token::Error, token_span.into()),
//        });
//
//    token_iter
//}
