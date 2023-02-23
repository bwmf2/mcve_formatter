// ---------------------------------------------------------
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::upper_case_acronyms)]

#[allow(unused_imports)]
#[allow(unused_imports)]
use std::marker::PhantomData;

// -------------------------------------------------------------------------------------------------
//
// Output Types of productions deduced from the structure of the transformed grammar
//

///
/// Type derived for production 277
///
/// Expression11OptGroup: UnaryOperator;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11OptGroupUnaryOperator {
    pub unary_operator: Box<UnaryOperator>,
}

///
/// Type derived for production 278
///
/// Expression11OptGroup: Operator09;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11OptGroupOperator09 {
    pub operator09: Box<Operator09>,
}

///
/// Type derived for production 279
///
/// Expression11OptGroup: Operator05;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11OptGroupOperator05 {
    pub operator05: Box<Operator05>,
}

///
/// Type derived for production 280
///
/// Expression11OptGroup: Operator03;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11OptGroupOperator03 {
    pub operator03: Box<Operator03>,
}

///
/// Type derived for production 281
///
/// Expression11OptGroup: Operator04;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11OptGroupOperator04 {
    pub operator04: Box<Operator04>,
}

///
/// Type derived for production 283
///
/// Factor: Number;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorNumber {
    pub number: Box<Number>,
}

///
/// Type derived for production 284
///
/// Factor: FactorOpt /* Option */ HierarchicalIdentifier FactorOpt0 /* Option */;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorFactorOptHierarchicalIdentifierFactorOpt0 {
    pub factor_opt: Option<Box<FactorOpt>>,
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
    pub factor_opt0: Option<Box<FactorOpt0>>,
}

///
/// Type derived for production 285
///
/// Factor: LParen Expression RParen;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorLParenExpressionRParen {
    pub l_paren: Box<LParen>,
    pub expression: Box<Expression>,
    pub r_paren: Box<RParen>,
}

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

///
/// Type derived for non-terminal Comma
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Comma {}

///
/// Type derived for non-terminal Dollar
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Dollar {}

///
/// Type derived for non-terminal Dot
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Dot {}

///
/// Type derived for non-terminal Expression
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression {
    pub expression01: Box<Expression01>,
    pub expression_list: Vec<ExpressionList>,
}

///
/// Type derived for non-terminal Expression01
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression01 {
    pub expression02: Box<Expression02>,
    pub expression01_list: Vec<Expression01List>,
}

///
/// Type derived for non-terminal Expression01List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression01List {
    pub operator02: Box<Operator02>,
    pub expression02: Box<Expression02>,
}

///
/// Type derived for non-terminal Expression02
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression02 {
    pub expression03: Box<Expression03>,
    pub expression02_list: Vec<Expression02List>,
}

///
/// Type derived for non-terminal Expression02List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression02List {
    pub operator03: Box<Operator03>,
    pub expression03: Box<Expression03>,
}

///
/// Type derived for non-terminal Expression03
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression03 {
    pub expression04: Box<Expression04>,
    pub expression03_list: Vec<Expression03List>,
}

///
/// Type derived for non-terminal Expression03List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression03List {
    pub operator04: Box<Operator04>,
    pub expression04: Box<Expression04>,
}

///
/// Type derived for non-terminal Expression04
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression04 {
    pub expression05: Box<Expression05>,
    pub expression04_list: Vec<Expression04List>,
}

///
/// Type derived for non-terminal Expression04List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression04List {
    pub operator05: Box<Operator05>,
    pub expression05: Box<Expression05>,
}

///
/// Type derived for non-terminal Expression05
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression05 {
    pub expression06: Box<Expression06>,
    pub expression05_list: Vec<Expression05List>,
}

///
/// Type derived for non-terminal Expression05List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression05List {
    pub operator06: Box<Operator06>,
    pub expression06: Box<Expression06>,
}

///
/// Type derived for non-terminal Expression06
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression06 {
    pub expression07: Box<Expression07>,
    pub expression06_list: Vec<Expression06List>,
}

///
/// Type derived for non-terminal Expression06List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression06List {
    pub operator07: Box<Operator07>,
    pub expression07: Box<Expression07>,
}

///
/// Type derived for non-terminal Expression07
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression07 {
    pub expression08: Box<Expression08>,
    pub expression07_list: Vec<Expression07List>,
}

///
/// Type derived for non-terminal Expression07List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression07List {
    pub operator08: Box<Operator08>,
    pub expression08: Box<Expression08>,
}

///
/// Type derived for non-terminal Expression08
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression08 {
    pub expression09: Box<Expression09>,
    pub expression08_list: Vec<Expression08List>,
}

///
/// Type derived for non-terminal Expression08List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression08List {
    pub operator09: Box<Operator09>,
    pub expression09: Box<Expression09>,
}

///
/// Type derived for non-terminal Expression09
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression09 {
    pub expression10: Box<Expression10>,
    pub expression09_list: Vec<Expression09List>,
}

///
/// Type derived for non-terminal Expression09List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression09List {
    pub operator10: Box<Operator10>,
    pub expression10: Box<Expression10>,
}

///
/// Type derived for non-terminal Expression10
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression10 {
    pub expression11: Box<Expression11>,
    pub expression10_list: Vec<Expression10List>,
}

///
/// Type derived for non-terminal Expression10List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression10List {
    pub operator11: Box<Operator11>,
    pub expression11: Box<Expression11>,
}

///
/// Type derived for non-terminal Expression11
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11 {
    pub expression11_opt: Option<Box<Expression11Opt>>,
    pub factor: Box<Factor>,
}

///
/// Type derived for non-terminal Expression11Opt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expression11Opt {
    pub expression11_opt_group: Box<Expression11OptGroup>,
}

///
/// Type derived for non-terminal Expression11OptGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression11OptGroup {
    UnaryOperator(Expression11OptGroupUnaryOperator),
    Operator09(Expression11OptGroupOperator09),
    Operator05(Expression11OptGroupOperator05),
    Operator03(Expression11OptGroupOperator03),
    Operator04(Expression11OptGroupOperator04),
}

///
/// Type derived for non-terminal ExpressionList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExpressionList {
    pub operator01: Box<Operator01>,
    pub expression01: Box<Expression01>,
}

///
/// Type derived for non-terminal Factor
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Factor {
    Number(FactorNumber),
    FactorOptHierarchicalIdentifierFactorOpt0(FactorFactorOptHierarchicalIdentifierFactorOpt0),
    LParenExpressionRParen(FactorLParenExpressionRParen),
}

///
/// Type derived for non-terminal FactorOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorOpt {
    pub dollar: Box<Dollar>,
}

///
/// Type derived for non-terminal FactorOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorOpt0 {
    pub l_paren: Box<LParen>,
    pub factor_opt1: Option<Box<FactorOpt1>>,
    pub r_paren: Box<RParen>,
}

///
/// Type derived for non-terminal FactorOpt1
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorOpt1 {
    pub function_call_arg: Box<FunctionCallArg>,
}

///
/// Type derived for non-terminal FunctionCallArg
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionCallArg {
    pub expression: Box<Expression>,
    pub function_call_arg_list: Vec<FunctionCallArgList>,
    pub function_call_arg_opt: Option<Box<FunctionCallArgOpt>>,
}

///
/// Type derived for non-terminal FunctionCallArgList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionCallArgList {
    pub comma: Box<Comma>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal FunctionCallArgOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionCallArgOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal HierarchicalIdentifier
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HierarchicalIdentifier {
    pub identifier: Box<Identifier>,
    pub hierarchical_identifier_list: Vec<HierarchicalIdentifierList>,
    pub hierarchical_identifier_list0: Vec<HierarchicalIdentifierList0>,
}

///
/// Type derived for non-terminal HierarchicalIdentifierList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HierarchicalIdentifierList {
    pub range: Box<Range>,
}

///
/// Type derived for non-terminal HierarchicalIdentifierList0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HierarchicalIdentifierList0 {
    pub dot: Box<Dot>,
    pub identifier: Box<Identifier>,
    pub hierarchical_identifier_list0_list: Vec<HierarchicalIdentifierList0List>,
}

///
/// Type derived for non-terminal HierarchicalIdentifierList0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HierarchicalIdentifierList0List {
    pub range: Box<Range>,
}

///
/// Type derived for non-terminal Identifier
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {}

///
/// Type derived for non-terminal LBracket
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBracket {}

///
/// Type derived for non-terminal LParen
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LParen {
    pub l_paren_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Number
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Number {}

///
/// Type derived for non-terminal Operator01
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator01 {
    pub operator01_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator02
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator02 {
    pub operator02_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator03
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator03 {
    pub operator03_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator04
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator04 {
    pub operator04_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator05
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator05 {
    pub operator05_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator06
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator06 {
    pub operator06_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator07
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator07 {
    pub operator07_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator08
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator08 {
    pub operator08_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator09
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator09 {
    pub operator09_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator10
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator10 {
    pub operator10_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator11
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator11 {
    pub operator11_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal RBracket
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBracket {}

///
/// Type derived for non-terminal RParen
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RParen {
    pub r_paren_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Range
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Range {
    pub l_bracket: Box<LBracket>,
    pub expression: Box<Expression>,
    pub range_opt: Option<Box<RangeOpt>>,
    pub r_bracket: Box<RBracket>,
}

///
/// Type derived for non-terminal RangeOperator
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RangeOperator {}

///
/// Type derived for non-terminal RangeOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RangeOpt {
    pub range_operator: Box<RangeOperator>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal UnaryOperator
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnaryOperator {
    pub unary_operator_token: crate::veryl_token::VerylToken,
}
