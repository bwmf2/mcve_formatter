#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct VerylToken {
    pub token: Token,
    pub comments: Vec<Token>,
}
