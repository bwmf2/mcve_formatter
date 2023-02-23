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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Empty;

// -------------------------------------------------------------------------------------------------
//
// Output Types of productions deduced from the structure of the transformed grammar
//

#[derive(Debug, Clone)]
pub struct Expression11OptGroupOperator {
    pub operator: Box<Operator>,
}

///
/// Type derived for production 284
///
/// Factor: FactorOpt /* Option */ HierarchicalIdentifier FactorOpt0 /* Option */;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorFactorOptHierarchicalIdentifierFactorOpt0 {
    pub factor_opt: Option<Box<FactorOpt>>, // Cannot remove it.
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
    pub l_paren: Box<Empty>,
    pub expression: Box<Expression>,
    pub r_paren: Box<Empty>,
}

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

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
    pub operator02: Box<Operator>,
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
    pub operator03: Box<Operator>,
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
    pub operator04: Box<Operator>,
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
    pub operator05: Box<Operator>,
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
    pub operator06: Box<Operator>,
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
    pub operator07: Box<Operator>,
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
    pub operator08: Box<Operator>,
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
    pub operator09: Box<Operator>,
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
    pub operator10: Box<Operator>,
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
    pub operator11: Box<Operator>,
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
    Operator09(Expression11OptGroupOperator),
    Operator05(Expression11OptGroupOperator),
    Operator03(Expression11OptGroupOperator),
    Operator04(Expression11OptGroupOperator),
}

///
/// Type derived for non-terminal ExpressionList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExpressionList {
    pub operator01: Box<Operator>,
    pub expression01: Box<Expression01>,
}

///
/// Type derived for non-terminal Factor
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Factor {
    FactorOptHierarchicalIdentifierFactorOpt0(FactorFactorOptHierarchicalIdentifierFactorOpt0),
    LParenExpressionRParen(FactorLParenExpressionRParen),
}

///
/// Type derived for non-terminal FactorOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorOpt {
    pub dollar: Box<Empty>,
}

///
/// Type derived for non-terminal FactorOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorOpt0 {
    pub l_paren: Box<Empty>,
    pub factor_opt1: Option<Box<FactorOpt1>>,
    pub r_paren: Box<Empty>,
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
}

///
/// Type derived for non-terminal FunctionCallArgList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionCallArgList {
    pub comma: Box<Empty>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal HierarchicalIdentifier
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HierarchicalIdentifier {
    pub hierarchical_identifier_list: Vec<Box<Range>>,
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub operator_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Range
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Range {
    pub expression: Box<Expression>,
}
