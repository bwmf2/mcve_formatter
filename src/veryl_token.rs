#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub text: usize,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub pos: usize,
    pub file_path: usize,
}

#[derive(Debug, Clone)]
pub struct VerylToken {
    pub token: Token,
    pub comments: Vec<Token>,
}
