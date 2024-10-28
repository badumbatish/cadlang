use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    /// Decl of a variable
    #[token("let")]
    Let,

    /// A floating point number, this is from Logos (https://logos.maciej.codes/examples/json.html?highlight=number#json-parser)
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,
}
