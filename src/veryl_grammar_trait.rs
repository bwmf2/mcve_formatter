pub struct Empty;

pub struct Expression11OptGroupOperator {
    pub operator: Box<Operator>,
}

pub struct FactorFactorOptHierarchicalIdentifierFactorOpt0 {
    pub factor_opt: Option<Box<FactorOpt>>, // Cannot remove it.
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
    pub factor_opt0: Option<Box<FactorOpt0>>,
}

pub struct FactorLParenExpressionRParen {
    pub l_paren: Box<Empty>,
    pub expression: Box<Expression>,
    pub r_paren: Box<Empty>,
}

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
    pub expression11_opt: Option<Box<Expression11Opt>>,
    pub factor: Box<Factor>,
}

pub struct Expression11Opt {
    pub expression11_opt_group: Box<Expression11OptGroup>,
}

pub enum Expression11OptGroup {
    Operator09(Expression11OptGroupOperator),
    Operator05(Expression11OptGroupOperator),
    Operator03(Expression11OptGroupOperator),
    Operator04(Expression11OptGroupOperator),
}

pub enum Factor {
    FactorOptHierarchicalIdentifierFactorOpt0(FactorFactorOptHierarchicalIdentifierFactorOpt0),
    LParenExpressionRParen(FactorLParenExpressionRParen),
}

pub struct FactorOpt {
    pub dollar: Box<Empty>,
}

pub struct FactorOpt0 {
    pub l_paren: Box<Empty>,
    pub factor_opt1: Option<Box<Box<Box<Expression>>>>,
    pub r_paren: Box<Empty>,
}

pub struct HierarchicalIdentifier {
    pub hierarchical_identifier_list: Vec<Box<Box<Expression>>>,
}

pub struct Operator {
    pub operator_token: crate::veryl_token::VerylToken,
}
