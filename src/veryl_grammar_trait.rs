pub struct Expression {
    pub expression01: Box<Expression01>,
    pub expression_list: Vec<ExpressionList>,
}

pub struct ExpressionList {
    pub operator01: Box<Operator>,
    pub expression01: Box<Expression01>,
}

pub struct Expression01 {
    pub expression02: Box<Expression02>,
    pub expression01_list: Vec<Expression01List>,
}

pub struct Expression01List {
    pub operator02: Box<Operator>,
    pub expression02: Box<Expression02>,
}

pub struct Expression02 {
    pub expression03: Box<Expression03>,
    pub expression02_list: Vec<Expression02List>,
}

pub struct Expression02List {
    pub operator03: Box<Operator>,
    pub expression03: Box<Expression03>,
}

pub struct Expression03 {
    pub expression04: Box<Expression04>,
    pub expression03_list: Vec<Expression03List>,
}

pub struct Expression03List {
    pub operator04: Box<Operator>,
    pub expression04: Box<Expression04>,
}

pub struct Expression04 {
    pub expression05: Box<Expression05>,
    pub expression04_list: Vec<Expression04List>,
}

pub struct Expression04List {
    pub operator05: Box<Operator>,
    pub expression05: Box<Expression05>,
}

pub struct Expression05 {
    pub expression06: Box<Expression06>,
    pub expression05_list: Vec<Expression05List>,
}

pub struct Expression05List {
    pub operator06: Box<Operator>,
    pub expression06: Box<Expression06>,
}

pub struct Expression06 {
    pub expression07: Box<Expression07>,
    pub expression06_list: Vec<Expression06List>,
}

pub struct Expression06List {
    pub operator07: Box<Operator>,
    pub expression07: Box<Expression07>,
}

pub struct Expression07 {
    pub expression08: Box<Expression08>,
    pub expression07_list: Vec<Expression07List>,
}

pub struct Expression07List {
    pub operator08: Box<Operator>,
    pub expression08: Box<Expression08>,
}

pub struct Expression08 {
    pub expression09: Box<Expression09>,
    pub expression08_list: Vec<Expression08List>,
}

pub struct Expression08List {
    pub operator09: Box<Operator>,
    pub expression09: Box<Expression09>,
}

pub struct Expression09 {
    pub expression10: Box<Expression10>,
    pub expression09_list: Vec<Expression09List>,
}

pub struct Expression09List {
    pub operator10: Box<Operator>,
    pub expression10: Box<Expression10>,
}

pub struct Expression10 {
    pub expression11: Box<Expression11>,
    pub expression10_list: Vec<Expression10List>,
}

pub struct Expression10List {
    pub operator11: Box<Operator>,
    pub expression11: Box<Expression11>,
}

pub struct Expression11 {
    pub expression11_opt: Box<Expression11OptGroup>,
    pub factor: Box<Factor>,
}

pub enum Expression11OptGroup {
    Operator09(Box<Operator>),
    Operator05(Box<Operator>),
    Operator03(Box<Operator>),
    Operator04(Box<Operator>),
}

pub enum Factor {
    A(Vec<Box<Expression>>),
    B(Vec<Box<Expression>>),
    C(Vec<Box<Expression>>),
    // Faster
    // A(Option<Box<Option<Box<Expression>>>>),
    // B(Option<Box<Option<Box<Expression>>>>),
    // C(Option<Box<Option<Box<Expression>>>>),
    // D(Option<Box<Option<Box<Expression>>>>),
}

pub struct Operator {
    pub operator_token: crate::veryl_token::VerylToken,
}
