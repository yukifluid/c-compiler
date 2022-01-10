#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    OP(&'a str),
    NUM(&'a str),
    EOF
}
