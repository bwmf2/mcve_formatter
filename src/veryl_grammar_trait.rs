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
/// Type derived for production 228
///
/// Number: IntegralNumber;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberIntegralNumber {
    pub integral_number: Box<IntegralNumber>,
}

///
/// Type derived for production 229
///
/// Number: RealNumber;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberRealNumber {
    pub real_number: Box<RealNumber>,
}

///
/// Type derived for production 230
///
/// IntegralNumber: Based;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegralNumberBased {
    pub based: Box<Based>,
}

///
/// Type derived for production 231
///
/// IntegralNumber: BaseLess;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegralNumberBaseLess {
    pub base_less: Box<BaseLess>,
}

///
/// Type derived for production 232
///
/// IntegralNumber: AllBit;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntegralNumberAllBit {
    pub all_bit: Box<AllBit>,
}

///
/// Type derived for production 233
///
/// RealNumber: FixedPoint;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RealNumberFixedPoint {
    pub fixed_point: Box<FixedPoint>,
}

///
/// Type derived for production 234
///
/// RealNumber: Exponent;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RealNumberExponent {
    pub exponent: Box<Exponent>,
}

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

///
/// Type derived for production 300
///
/// RangeOperator: Colon;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RangeOperatorColon {
    pub colon: Box<Colon>,
}

///
/// Type derived for production 301
///
/// RangeOperator: PlusColon;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RangeOperatorPlusColon {
    pub plus_colon: Box<PlusColon>,
}

///
/// Type derived for production 302
///
/// RangeOperator: MinusColon;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RangeOperatorMinusColon {
    pub minus_colon: Box<MinusColon>,
}

///
/// Type derived for production 303
///
/// RangeOperator: Step;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RangeOperatorStep {
    pub step: Box<Step>,
}

///
/// Type derived for production 305
///
/// BuiltinType: Logic;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeLogic {
    pub logic: Box<Logic>,
}

///
/// Type derived for production 306
///
/// BuiltinType: Bit;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeBit {
    pub bit: Box<Bit>,
}

///
/// Type derived for production 307
///
/// BuiltinType: U32;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeU32 {
    pub u32: Box<U32>,
}

///
/// Type derived for production 308
///
/// BuiltinType: U64;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeU64 {
    pub u64: Box<U64>,
}

///
/// Type derived for production 309
///
/// BuiltinType: I32;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeI32 {
    pub i32: Box<I32>,
}

///
/// Type derived for production 310
///
/// BuiltinType: I64;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeI64 {
    pub i64: Box<I64>,
}

///
/// Type derived for production 311
///
/// BuiltinType: F32;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeF32 {
    pub f32: Box<F32>,
}

///
/// Type derived for production 312
///
/// BuiltinType: F64;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuiltinTypeF64 {
    pub f64: Box<F64>,
}

///
/// Type derived for production 314
///
/// TypeGroup: BuiltinType;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeGroupBuiltinType {
    pub builtin_type: Box<BuiltinType>,
}

///
/// Type derived for production 315
///
/// TypeGroup: Identifier;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeGroupIdentifier {
    pub identifier: Box<Identifier>,
}

///
/// Type derived for production 318
///
/// Statement: AssignmentStatement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatementAssignmentStatement {
    pub assignment_statement: Box<AssignmentStatement>,
}

///
/// Type derived for production 319
///
/// Statement: IfStatement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatementIfStatement {
    pub if_statement: Box<IfStatement>,
}

///
/// Type derived for production 320
///
/// Statement: IfResetStatement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatementIfResetStatement {
    pub if_reset_statement: Box<IfResetStatement>,
}

///
/// Type derived for production 321
///
/// Statement: ReturnStatement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatementReturnStatement {
    pub return_statement: Box<ReturnStatement>,
}

///
/// Type derived for production 322
///
/// Statement: ForStatement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatementForStatement {
    pub for_statement: Box<ForStatement>,
}

///
/// Type derived for production 324
///
/// AssignmentStatementGroup: Equ;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentStatementGroupEqu {
    pub equ: Box<Equ>,
}

///
/// Type derived for production 325
///
/// AssignmentStatementGroup: AssignmentOperator;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentStatementGroupAssignmentOperator {
    pub assignment_operator: Box<AssignmentOperator>,
}

///
/// Type derived for production 355
///
/// LetDeclarationGroup: VariableDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetDeclarationGroupVariableDeclaration {
    pub variable_declaration: Box<VariableDeclaration>,
}

///
/// Type derived for production 356
///
/// LetDeclarationGroup: InstanceDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetDeclarationGroupInstanceDeclaration {
    pub instance_declaration: Box<InstanceDeclaration>,
}

///
/// Type derived for production 369
///
/// AlwaysFfClockOptGroup: Posedge;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfClockOptGroupPosedge {
    pub posedge: Box<Posedge>,
}

///
/// Type derived for production 370
///
/// AlwaysFfClockOptGroup: Negedge;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfClockOptGroupNegedge {
    pub negedge: Box<Negedge>,
}

///
/// Type derived for production 374
///
/// AlwaysFfResetOptGroup: AsyncLow;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfResetOptGroupAsyncLow {
    pub async_low: Box<AsyncLow>,
}

///
/// Type derived for production 375
///
/// AlwaysFfResetOptGroup: AsyncHigh;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfResetOptGroupAsyncHigh {
    pub async_high: Box<AsyncHigh>,
}

///
/// Type derived for production 376
///
/// AlwaysFfResetOptGroup: SyncLow;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfResetOptGroupSyncLow {
    pub sync_low: Box<SyncLow>,
}

///
/// Type derived for production 377
///
/// AlwaysFfResetOptGroup: SyncHigh;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfResetOptGroupSyncHigh {
    pub sync_high: Box<SyncHigh>,
}

///
/// Type derived for production 443
///
/// WithParameterItemGroup: Parameter;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterItemGroupParameter {
    pub parameter: Box<Parameter>,
}

///
/// Type derived for production 444
///
/// WithParameterItemGroup: Localparam;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterItemGroupLocalparam {
    pub localparam: Box<Localparam>,
}

///
/// Type derived for production 454
///
/// Direction: Input;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectionInput {
    pub input: Box<Input>,
}

///
/// Type derived for production 455
///
/// Direction: Output;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectionOutput {
    pub output: Box<Output>,
}

///
/// Type derived for production 456
///
/// Direction: Inout;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectionInout {
    pub inout: Box<Inout>,
}

///
/// Type derived for production 457
///
/// Direction: Ref;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectionRef {
    pub r#ref: Box<Ref>,
}

///
/// Type derived for production 465
///
/// FunctionItem: LetDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionItemLetDeclaration {
    pub let_declaration: Box<LetDeclaration>,
}

///
/// Type derived for production 466
///
/// FunctionItem: Statement;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionItemStatement {
    pub statement: Box<Statement>,
}

///
/// Type derived for production 494
///
/// ModuleItem: LetDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemLetDeclaration {
    pub let_declaration: Box<LetDeclaration>,
}

///
/// Type derived for production 495
///
/// ModuleItem: ParameterDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemParameterDeclaration {
    pub parameter_declaration: Box<ParameterDeclaration>,
}

///
/// Type derived for production 496
///
/// ModuleItem: LocalparamDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemLocalparamDeclaration {
    pub localparam_declaration: Box<LocalparamDeclaration>,
}

///
/// Type derived for production 497
///
/// ModuleItem: AlwaysFfDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemAlwaysFfDeclaration {
    pub always_ff_declaration: Box<AlwaysFfDeclaration>,
}

///
/// Type derived for production 498
///
/// ModuleItem: AlwaysCombDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemAlwaysCombDeclaration {
    pub always_comb_declaration: Box<AlwaysCombDeclaration>,
}

///
/// Type derived for production 499
///
/// ModuleItem: AssignDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemAssignDeclaration {
    pub assign_declaration: Box<AssignDeclaration>,
}

///
/// Type derived for production 500
///
/// ModuleItem: FunctionDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemFunctionDeclaration {
    pub function_declaration: Box<FunctionDeclaration>,
}

///
/// Type derived for production 501
///
/// ModuleItem: ModuleIfDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemModuleIfDeclaration {
    pub module_if_declaration: Box<ModuleIfDeclaration>,
}

///
/// Type derived for production 502
///
/// ModuleItem: ModuleForDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemModuleForDeclaration {
    pub module_for_declaration: Box<ModuleForDeclaration>,
}

///
/// Type derived for production 503
///
/// ModuleItem: EnumDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemEnumDeclaration {
    pub enum_declaration: Box<EnumDeclaration>,
}

///
/// Type derived for production 504
///
/// ModuleItem: StructDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleItemStructDeclaration {
    pub struct_declaration: Box<StructDeclaration>,
}

///
/// Type derived for production 530
///
/// InterfaceItem: LetDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemLetDeclaration {
    pub let_declaration: Box<LetDeclaration>,
}

///
/// Type derived for production 531
///
/// InterfaceItem: ParameterDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemParameterDeclaration {
    pub parameter_declaration: Box<ParameterDeclaration>,
}

///
/// Type derived for production 532
///
/// InterfaceItem: LocalparamDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemLocalparamDeclaration {
    pub localparam_declaration: Box<LocalparamDeclaration>,
}

///
/// Type derived for production 533
///
/// InterfaceItem: ModportDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemModportDeclaration {
    pub modport_declaration: Box<ModportDeclaration>,
}

///
/// Type derived for production 534
///
/// InterfaceItem: InterfaceIfDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemInterfaceIfDeclaration {
    pub interface_if_declaration: Box<InterfaceIfDeclaration>,
}

///
/// Type derived for production 535
///
/// InterfaceItem: InterfaceForDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemInterfaceForDeclaration {
    pub interface_for_declaration: Box<InterfaceForDeclaration>,
}

///
/// Type derived for production 536
///
/// InterfaceItem: EnumDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemEnumDeclaration {
    pub enum_declaration: Box<EnumDeclaration>,
}

///
/// Type derived for production 537
///
/// InterfaceItem: StructDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceItemStructDeclaration {
    pub struct_declaration: Box<StructDeclaration>,
}

///
/// Type derived for production 538
///
/// Description: ModuleDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DescriptionModuleDeclaration {
    pub module_declaration: Box<ModuleDeclaration>,
}

///
/// Type derived for production 539
///
/// Description: InterfaceDeclaration;
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DescriptionInterfaceDeclaration {
    pub interface_declaration: Box<InterfaceDeclaration>,
}

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

///
/// Type derived for non-terminal AllBit
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AllBit {
    pub all_bit_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AllBitTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AllBitTerm {
    pub all_bit_term: crate::veryl_token::Token, /* '[01] */
}

///
/// Type derived for non-terminal AllBitToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AllBitToken {
    pub all_bit_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal AlwaysComb
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysComb {
    pub always_comb_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AlwaysCombDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysCombDeclaration {
    pub always_comb: Box<AlwaysComb>,
    pub l_brace: Box<LBrace>,
    pub always_comb_declaration_list: Vec<AlwaysCombDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal AlwaysCombDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysCombDeclarationList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal AlwaysCombTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysCombTerm {
    pub always_comb_term: crate::veryl_token::Token, /* \balways_comb\b */
}

///
/// Type derived for non-terminal AlwaysCombToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysCombToken {
    pub always_comb_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal AlwaysFf
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFf {
    pub always_ff_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AlwaysFfClock
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfClock {
    pub always_ff_clock_opt: Option<Box<AlwaysFfClockOpt>>,
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
}

///
/// Type derived for non-terminal AlwaysFfClockOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfClockOpt {
    pub always_ff_clock_opt_group: Box<AlwaysFfClockOptGroup>,
}

///
/// Type derived for non-terminal AlwaysFfClockOptGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AlwaysFfClockOptGroup {
    Posedge(AlwaysFfClockOptGroupPosedge),
    Negedge(AlwaysFfClockOptGroupNegedge),
}

///
/// Type derived for non-terminal AlwaysFfDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfDeclaration {
    pub always_ff: Box<AlwaysFf>,
    pub l_paren: Box<LParen>,
    pub always_ff_clock: Box<AlwaysFfClock>,
    pub always_ff_declaration_opt: Option<Box<AlwaysFfDeclarationOpt>>,
    pub r_paren: Box<RParen>,
    pub l_brace: Box<LBrace>,
    pub always_ff_declaration_list: Vec<AlwaysFfDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal AlwaysFfDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfDeclarationList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal AlwaysFfDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfDeclarationOpt {
    pub comma: Box<Comma>,
    pub always_ff_reset: Box<AlwaysFfReset>,
}

///
/// Type derived for non-terminal AlwaysFfReset
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfReset {
    pub always_ff_reset_opt: Option<Box<AlwaysFfResetOpt>>,
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
}

///
/// Type derived for non-terminal AlwaysFfResetOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfResetOpt {
    pub always_ff_reset_opt_group: Box<AlwaysFfResetOptGroup>,
}

///
/// Type derived for non-terminal AlwaysFfResetOptGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AlwaysFfResetOptGroup {
    AsyncLow(AlwaysFfResetOptGroupAsyncLow),
    AsyncHigh(AlwaysFfResetOptGroupAsyncHigh),
    SyncLow(AlwaysFfResetOptGroupSyncLow),
    SyncHigh(AlwaysFfResetOptGroupSyncHigh),
}

///
/// Type derived for non-terminal AlwaysFfTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfTerm {
    pub always_ff_term: crate::veryl_token::Token, /* \balways_ff\b */
}

///
/// Type derived for non-terminal AlwaysFfToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlwaysFfToken {
    pub always_ff_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Assign
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Assign {
    pub assign_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AssignDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignDeclaration {
    pub assign: Box<Assign>,
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal AssignTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignTerm {
    pub assign_term: crate::veryl_token::Token, /* \bassign\b */
}

///
/// Type derived for non-terminal AssignToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignToken {
    pub assign_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal AssignmentOperator
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentOperator {
    pub assignment_operator_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AssignmentOperatorTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentOperatorTerm {
    pub assignment_operator_term: crate::veryl_token::Token, /* \+=|-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>=|<<<=|>>>= */
}

///
/// Type derived for non-terminal AssignmentOperatorToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentOperatorToken {
    pub assignment_operator_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal AssignmentStatement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub hierarchical_identifier: Box<HierarchicalIdentifier>,
    pub assignment_statement_group: Box<AssignmentStatementGroup>,
    pub expression: Box<Expression>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal AssignmentStatementGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AssignmentStatementGroup {
    Equ(AssignmentStatementGroupEqu),
    AssignmentOperator(AssignmentStatementGroupAssignmentOperator),
}

///
/// Type derived for non-terminal AsyncHigh
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncHigh {
    pub async_high_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AsyncHighTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncHighTerm {
    pub async_high_term: crate::veryl_token::Token, /* \basync_high\b */
}

///
/// Type derived for non-terminal AsyncHighToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncHighToken {
    pub async_high_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal AsyncLow
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncLow {
    pub async_low_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal AsyncLowTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncLowTerm {
    pub async_low_term: crate::veryl_token::Token, /* \basync_low\b */
}

///
/// Type derived for non-terminal AsyncLowToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AsyncLowToken {
    pub async_low_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal BaseLess
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaseLess {
    pub base_less_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal BaseLessTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaseLessTerm {
    pub base_less_term: crate::veryl_token::Token, /* [0-9]+(?:_[0-9]+)* */
}

///
/// Type derived for non-terminal BaseLessToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaseLessToken {
    pub base_less_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Based
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Based {
    pub based_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal BasedTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BasedTerm {
    pub based_term: crate::veryl_token::Token, /* [0-9]+(?:_[0-9]+)*'[bodh][0-9a-fA-FxzXZ]+(?:_[0-9a-fA-FxzXZ]+)* */
}

///
/// Type derived for non-terminal BasedToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BasedToken {
    pub based_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Bit
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Bit {
    pub bit_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal BitTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BitTerm {
    pub bit_term: crate::veryl_token::Token, /* \bbit\b */
}

///
/// Type derived for non-terminal BitToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BitToken {
    pub bit_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal BuiltinType
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BuiltinType {
    Logic(BuiltinTypeLogic),
    Bit(BuiltinTypeBit),
    U32(BuiltinTypeU32),
    U64(BuiltinTypeU64),
    I32(BuiltinTypeI32),
    I64(BuiltinTypeI64),
    F32(BuiltinTypeF32),
    F64(BuiltinTypeF64),
}

///
/// Type derived for non-terminal Colon
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Colon {
    pub colon_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ColonTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColonTerm {
    pub colon_term: crate::veryl_token::Token, /* : */
}

///
/// Type derived for non-terminal ColonToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColonToken {
    pub colon_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Comma
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Comma {
    pub comma_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal CommaTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommaTerm {
    pub comma_term: crate::veryl_token::Token, /* , */
}

///
/// Type derived for non-terminal CommaToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommaToken {
    pub comma_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Comments
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Comments {
    pub comments_opt: Option<Box<CommentsOpt>>,
}

///
/// Type derived for non-terminal CommentsOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommentsOpt {
    pub comments_term: Box<CommentsTerm>,
}

///
/// Type derived for non-terminal CommentsTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommentsTerm {
    pub comments_term: crate::veryl_token::Token, /* (?:(?:(?://.*(?:\r\n|\r|\n|$))|(?:(?ms)/\u{2a}.*?\u{2a}/))\s*)+ */
}

///
/// Type derived for non-terminal Description
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Description {
    ModuleDeclaration(DescriptionModuleDeclaration),
    InterfaceDeclaration(DescriptionInterfaceDeclaration),
}

///
/// Type derived for non-terminal Direction
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Direction {
    Input(DirectionInput),
    Output(DirectionOutput),
    Inout(DirectionInout),
    Ref(DirectionRef),
}

///
/// Type derived for non-terminal Dollar
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Dollar {
    pub dollar_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal DollarTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DollarTerm {
    pub dollar_term: crate::veryl_token::Token, /* $ */
}

///
/// Type derived for non-terminal DollarToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DollarToken {
    pub dollar_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Dot
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Dot {
    pub dot_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal DotDot
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DotDot {
    pub dot_dot_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal DotDotTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DotDotTerm {
    pub dot_dot_term: crate::veryl_token::Token, /* .. */
}

///
/// Type derived for non-terminal DotDotToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DotDotToken {
    pub dot_dot_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal DotTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DotTerm {
    pub dot_term: crate::veryl_token::Token, /* . */
}

///
/// Type derived for non-terminal DotToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DotToken {
    pub dot_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Else
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Else {
    pub else_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ElseTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElseTerm {
    pub else_term: crate::veryl_token::Token, /* \belse\b */
}

///
/// Type derived for non-terminal ElseToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElseToken {
    pub else_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Enum
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Enum {
    pub enum_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal EnumDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub r#enum: Box<Enum>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
    pub l_brace: Box<LBrace>,
    pub enum_list: Box<EnumList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal EnumItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumItem {
    pub identifier: Box<Identifier>,
    pub enum_item_opt: Option<Box<EnumItemOpt>>,
}

///
/// Type derived for non-terminal EnumItemOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumItemOpt {
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal EnumList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumList {
    pub enum_item: Box<EnumItem>,
    pub enum_list_list: Vec<EnumListList>,
    pub enum_list_opt: Option<Box<EnumListOpt>>,
}

///
/// Type derived for non-terminal EnumListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumListList {
    pub comma: Box<Comma>,
    pub enum_item: Box<EnumItem>,
}

///
/// Type derived for non-terminal EnumListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal EnumTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumTerm {
    pub enum_term: crate::veryl_token::Token, /* \benum\b */
}

///
/// Type derived for non-terminal EnumToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumToken {
    pub enum_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Equ
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Equ {
    pub equ_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal EquTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EquTerm {
    pub equ_term: crate::veryl_token::Token, /* = */
}

///
/// Type derived for non-terminal EquToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EquToken {
    pub equ_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Exponent
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Exponent {
    pub exponent_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ExponentTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExponentTerm {
    pub exponent_term: crate::veryl_token::Token, /* [0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)* */
}

///
/// Type derived for non-terminal ExponentToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExponentToken {
    pub exponent_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

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
/// Type derived for non-terminal F32
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F32 {
    pub f32_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal F32Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F32Term {
    pub f32_term: crate::veryl_token::Token, /* \bf32\b */
}

///
/// Type derived for non-terminal F32Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F32Token {
    pub f32_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal F64
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F64 {
    pub f64_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal F64Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F64Term {
    pub f64_term: crate::veryl_token::Token, /* \bf64\b */
}

///
/// Type derived for non-terminal F64Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct F64Token {
    pub f64_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal FixedPoint
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixedPoint {
    pub fixed_point_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal FixedPointTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixedPointTerm {
    pub fixed_point_term: crate::veryl_token::Token, /* [0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)* */
}

///
/// Type derived for non-terminal FixedPointToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixedPointToken {
    pub fixed_point_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal For
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct For {
    pub for_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ForStatement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub r#for: Box<For>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
    pub r#in: Box<In>,
    pub expression: Box<Expression>,
    pub dot_dot: Box<DotDot>,
    pub expression0: Box<Expression>,
    pub for_statement_opt: Option<Box<ForStatementOpt>>,
    pub l_brace: Box<LBrace>,
    pub for_statement_list: Vec<ForStatementList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ForStatementList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForStatementList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal ForStatementOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForStatementOpt {
    pub step: Box<Step>,
    pub assignment_operator: Box<AssignmentOperator>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal ForTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForTerm {
    pub for_term: crate::veryl_token::Token, /* \bfor\b */
}

///
/// Type derived for non-terminal ForToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForToken {
    pub for_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Function
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Function {
    pub function_token: crate::veryl_token::VerylToken,
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
/// Type derived for non-terminal FunctionDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub function: Box<Function>,
    pub identifier: Box<Identifier>,
    pub function_declaration_opt: Option<Box<FunctionDeclarationOpt>>,
    pub function_declaration_opt0: Option<Box<FunctionDeclarationOpt0>>,
    pub minus_g_t: Box<MinusGT>,
    pub r#type: Box<Type>,
    pub l_brace: Box<LBrace>,
    pub function_declaration_list: Vec<FunctionDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal FunctionDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclarationList {
    pub function_item: Box<FunctionItem>,
}

///
/// Type derived for non-terminal FunctionDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclarationOpt {
    pub with_parameter: Box<WithParameter>,
}

///
/// Type derived for non-terminal FunctionDeclarationOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclarationOpt0 {
    pub port_declaration: Box<PortDeclaration>,
}

///
/// Type derived for non-terminal FunctionItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunctionItem {
    LetDeclaration(FunctionItemLetDeclaration),
    Statement(FunctionItemStatement),
}

///
/// Type derived for non-terminal FunctionTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionTerm {
    pub function_term: crate::veryl_token::Token, /* \bfunction\b */
}

///
/// Type derived for non-terminal FunctionToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionToken {
    pub function_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Hash
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Hash {
    pub hash_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal HashTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HashTerm {
    pub hash_term: crate::veryl_token::Token, /* # */
}

///
/// Type derived for non-terminal HashToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HashToken {
    pub hash_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal I32
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I32 {
    pub i32_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal I32Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I32Term {
    pub i32_term: crate::veryl_token::Token, /* \bi32\b */
}

///
/// Type derived for non-terminal I32Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I32Token {
    pub i32_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal I64
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I64 {
    pub i64_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal I64Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I64Term {
    pub i64_term: crate::veryl_token::Token, /* \bi64\b */
}

///
/// Type derived for non-terminal I64Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I64Token {
    pub i64_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Identifier
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {
    pub identifier_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal IdentifierTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IdentifierTerm {
    pub identifier_term: crate::veryl_token::Token, /* [a-zA-Z_][0-9a-zA-Z_]* */
}

///
/// Type derived for non-terminal IdentifierToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IdentifierToken {
    pub identifier_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal If
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct If {
    pub if_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal IfReset
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfReset {
    pub if_reset_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal IfResetStatement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatement {
    pub if_reset: Box<IfReset>,
    pub l_brace: Box<LBrace>,
    pub if_reset_statement_list: Vec<IfResetStatementList>,
    pub r_brace: Box<RBrace>,
    pub if_reset_statement_list0: Vec<IfResetStatementList0>,
    pub if_reset_statement_opt: Option<Box<IfResetStatementOpt>>,
}

///
/// Type derived for non-terminal IfResetStatementList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatementList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfResetStatementList0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatementList0 {
    pub r#else: Box<Else>,
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub l_brace: Box<LBrace>,
    pub if_reset_statement_list0_list: Vec<IfResetStatementList0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal IfResetStatementList0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatementList0List {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfResetStatementOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatementOpt {
    pub r#else: Box<Else>,
    pub l_brace: Box<LBrace>,
    pub if_reset_statement_opt_list: Vec<IfResetStatementOptList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal IfResetStatementOptList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetStatementOptList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfResetTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetTerm {
    pub if_reset_term: crate::veryl_token::Token, /* \bif_reset\b */
}

///
/// Type derived for non-terminal IfResetToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfResetToken {
    pub if_reset_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal IfStatement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub l_brace: Box<LBrace>,
    pub if_statement_list: Vec<IfStatementList>,
    pub r_brace: Box<RBrace>,
    pub if_statement_list0: Vec<IfStatementList0>,
    pub if_statement_opt: Option<Box<IfStatementOpt>>,
}

///
/// Type derived for non-terminal IfStatementList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatementList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfStatementList0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatementList0 {
    pub r#else: Box<Else>,
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub l_brace: Box<LBrace>,
    pub if_statement_list0_list: Vec<IfStatementList0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal IfStatementList0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatementList0List {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfStatementOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatementOpt {
    pub r#else: Box<Else>,
    pub l_brace: Box<LBrace>,
    pub if_statement_opt_list: Vec<IfStatementOptList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal IfStatementOptList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfStatementOptList {
    pub statement: Box<Statement>,
}

///
/// Type derived for non-terminal IfTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfTerm {
    pub if_term: crate::veryl_token::Token, /* \bif\b */
}

///
/// Type derived for non-terminal IfToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IfToken {
    pub if_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal In
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct In {
    pub in_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal InTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InTerm {
    pub in_term: crate::veryl_token::Token, /* \bin\b */
}

///
/// Type derived for non-terminal InToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InToken {
    pub in_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Inout
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Inout {
    pub inout_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal InoutTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InoutTerm {
    pub inout_term: crate::veryl_token::Token, /* \binout\b */
}

///
/// Type derived for non-terminal InoutToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InoutToken {
    pub inout_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Input
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Input {
    pub input_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal InputTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InputTerm {
    pub input_term: crate::veryl_token::Token, /* \binput\b */
}

///
/// Type derived for non-terminal InputToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InputToken {
    pub input_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Inst
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Inst {
    pub inst_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal InstTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstTerm {
    pub inst_term: crate::veryl_token::Token, /* \binst\b */
}

///
/// Type derived for non-terminal InstToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstToken {
    pub inst_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal InstanceDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceDeclaration {
    pub inst: Box<Inst>,
    pub identifier: Box<Identifier>,
    pub instance_declaration_opt: Option<Box<InstanceDeclarationOpt>>,
    pub instance_declaration_opt0: Option<Box<InstanceDeclarationOpt0>>,
    pub instance_declaration_opt1: Option<Box<InstanceDeclarationOpt1>>,
}

///
/// Type derived for non-terminal InstanceDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceDeclarationOpt {
    pub width: Box<Width>,
}

///
/// Type derived for non-terminal InstanceDeclarationOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceDeclarationOpt0 {
    pub instance_parameter: Box<InstanceParameter>,
}

///
/// Type derived for non-terminal InstanceDeclarationOpt1
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceDeclarationOpt1 {
    pub l_brace: Box<LBrace>,
    pub instance_declaration_opt2: Option<Box<InstanceDeclarationOpt2>>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal InstanceDeclarationOpt2
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceDeclarationOpt2 {
    pub instance_port_list: Box<InstancePortList>,
}

///
/// Type derived for non-terminal InstanceParameter
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameter {
    pub hash: Box<Hash>,
    pub l_paren: Box<LParen>,
    pub instance_parameter_opt: Option<Box<InstanceParameterOpt>>,
    pub r_paren: Box<RParen>,
}

///
/// Type derived for non-terminal InstanceParameterItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterItem {
    pub identifier: Box<Identifier>,
    pub instance_parameter_item_opt: Option<Box<InstanceParameterItemOpt>>,
}

///
/// Type derived for non-terminal InstanceParameterItemOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterItemOpt {
    pub colon: Box<Colon>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal InstanceParameterList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterList {
    pub instance_parameter_item: Box<InstanceParameterItem>,
    pub instance_parameter_list_list: Vec<InstanceParameterListList>,
    pub instance_parameter_list_opt: Option<Box<InstanceParameterListOpt>>,
}

///
/// Type derived for non-terminal InstanceParameterListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterListList {
    pub comma: Box<Comma>,
    pub instance_parameter_item: Box<InstanceParameterItem>,
}

///
/// Type derived for non-terminal InstanceParameterListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal InstanceParameterOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceParameterOpt {
    pub instance_parameter_list: Box<InstanceParameterList>,
}

///
/// Type derived for non-terminal InstancePortItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancePortItem {
    pub identifier: Box<Identifier>,
    pub instance_port_item_opt: Option<Box<InstancePortItemOpt>>,
}

///
/// Type derived for non-terminal InstancePortItemOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancePortItemOpt {
    pub colon: Box<Colon>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal InstancePortList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancePortList {
    pub instance_port_item: Box<InstancePortItem>,
    pub instance_port_list_list: Vec<InstancePortListList>,
    pub instance_port_list_opt: Option<Box<InstancePortListOpt>>,
}

///
/// Type derived for non-terminal InstancePortListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancePortListList {
    pub comma: Box<Comma>,
    pub instance_port_item: Box<InstancePortItem>,
}

///
/// Type derived for non-terminal InstancePortListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancePortListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal IntegralNumber
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum IntegralNumber {
    Based(IntegralNumberBased),
    BaseLess(IntegralNumberBaseLess),
    AllBit(IntegralNumberAllBit),
}

///
/// Type derived for non-terminal Interface
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Interface {
    pub interface_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal InterfaceDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceDeclaration {
    pub interface: Box<Interface>,
    pub identifier: Box<Identifier>,
    pub interface_declaration_opt: Option<Box<InterfaceDeclarationOpt>>,
    pub l_brace: Box<LBrace>,
    pub interface_declaration_list: Vec<InterfaceDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal InterfaceDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceDeclarationList {
    pub interface_item: Box<InterfaceItem>,
}

///
/// Type derived for non-terminal InterfaceDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceDeclarationOpt {
    pub with_parameter: Box<WithParameter>,
}

///
/// Type derived for non-terminal InterfaceForDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceForDeclaration {
    pub r#for: Box<For>,
    pub identifier: Box<Identifier>,
    pub r#in: Box<In>,
    pub expression: Box<Expression>,
    pub dot_dot: Box<DotDot>,
    pub expression0: Box<Expression>,
    pub interface_for_declaration_opt: Option<Box<InterfaceForDeclarationOpt>>,
    pub colon: Box<Colon>,
    pub identifier0: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub interface_for_declaration_list: Vec<InterfaceForDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal InterfaceForDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceForDeclarationList {
    pub interface_item: Box<InterfaceItem>,
}

///
/// Type derived for non-terminal InterfaceForDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceForDeclarationOpt {
    pub step: Box<Step>,
    pub assignment_operator: Box<AssignmentOperator>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal InterfaceIfDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclaration {
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub interface_if_declaration_list: Vec<InterfaceIfDeclarationList>,
    pub r_brace: Box<RBrace>,
    pub interface_if_declaration_list0: Vec<InterfaceIfDeclarationList0>,
    pub interface_if_declaration_opt0: Option<Box<InterfaceIfDeclarationOpt0>>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationList {
    pub interface_item: Box<InterfaceItem>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationList0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationList0 {
    pub r#else: Box<Else>,
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub interface_if_declaration_opt: Option<Box<InterfaceIfDeclarationOpt>>,
    pub l_brace: Box<LBrace>,
    pub interface_if_declaration_list0_list: Vec<InterfaceIfDeclarationList0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationList0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationList0List {
    pub interface_item: Box<InterfaceItem>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationOpt {
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationOpt0 {
    pub r#else: Box<Else>,
    pub interface_if_declaration_opt1: Option<Box<InterfaceIfDeclarationOpt1>>,
    pub l_brace: Box<LBrace>,
    pub interface_if_declaration_opt0_list: Vec<InterfaceIfDeclarationOpt0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationOpt0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationOpt0List {
    pub interface_item: Box<InterfaceItem>,
}

///
/// Type derived for non-terminal InterfaceIfDeclarationOpt1
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceIfDeclarationOpt1 {
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
}

///
/// Type derived for non-terminal InterfaceItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InterfaceItem {
    LetDeclaration(InterfaceItemLetDeclaration),
    ParameterDeclaration(InterfaceItemParameterDeclaration),
    LocalparamDeclaration(InterfaceItemLocalparamDeclaration),
    ModportDeclaration(InterfaceItemModportDeclaration),
    InterfaceIfDeclaration(InterfaceItemInterfaceIfDeclaration),
    InterfaceForDeclaration(InterfaceItemInterfaceForDeclaration),
    EnumDeclaration(InterfaceItemEnumDeclaration),
    StructDeclaration(InterfaceItemStructDeclaration),
}

///
/// Type derived for non-terminal InterfaceTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceTerm {
    pub interface_term: crate::veryl_token::Token, /* \binterface\b */
}

///
/// Type derived for non-terminal InterfaceToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterfaceToken {
    pub interface_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal LBrace
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBrace {
    pub l_brace_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LBraceTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBraceTerm {
    pub l_brace_term: crate::veryl_token::Token, /* { */
}

///
/// Type derived for non-terminal LBraceToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBraceToken {
    pub l_brace_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal LBracket
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBracket {
    pub l_bracket_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LBracketTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBracketTerm {
    pub l_bracket_term: crate::veryl_token::Token, /* [ */
}

///
/// Type derived for non-terminal LBracketToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LBracketToken {
    pub l_bracket_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal LParen
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LParen {
    pub l_paren_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LParenTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LParenTerm {
    pub l_paren_term: crate::veryl_token::Token, /* ( */
}

///
/// Type derived for non-terminal LParenToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LParenToken {
    pub l_paren_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Let
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Let {
    pub let_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LetDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetDeclaration {
    pub r#let: Box<Let>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub let_declaration_group: Box<LetDeclarationGroup>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal LetDeclarationGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LetDeclarationGroup {
    VariableDeclaration(LetDeclarationGroupVariableDeclaration),
    InstanceDeclaration(LetDeclarationGroupInstanceDeclaration),
}

///
/// Type derived for non-terminal LetTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetTerm {
    pub let_term: crate::veryl_token::Token, /* \blet\b */
}

///
/// Type derived for non-terminal LetToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LetToken {
    pub let_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Localparam
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Localparam {
    pub localparam_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LocalparamDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalparamDeclaration {
    pub localparam: Box<Localparam>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal LocalparamTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalparamTerm {
    pub localparam_term: crate::veryl_token::Token, /* \blocalparam\b */
}

///
/// Type derived for non-terminal LocalparamToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalparamToken {
    pub localparam_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Logic
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Logic {
    pub logic_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal LogicTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogicTerm {
    pub logic_term: crate::veryl_token::Token, /* \blogic\b */
}

///
/// Type derived for non-terminal LogicToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogicToken {
    pub logic_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal MinusColon
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusColon {
    pub minus_colon_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal MinusColonTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusColonTerm {
    pub minus_colon_term: crate::veryl_token::Token, /* -: */
}

///
/// Type derived for non-terminal MinusColonToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusColonToken {
    pub minus_colon_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal MinusGT
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusGT {
    pub minus_g_t_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal MinusGTTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusGTTerm {
    pub minus_g_t_term: crate::veryl_token::Token, /* -> */
}

///
/// Type derived for non-terminal MinusGTToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinusGTToken {
    pub minus_g_t_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Modport
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Modport {
    pub modport_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ModportDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportDeclaration {
    pub modport: Box<Modport>,
    pub identifier: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub modport_list: Box<ModportList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ModportItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportItem {
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub direction: Box<Direction>,
}

///
/// Type derived for non-terminal ModportList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportList {
    pub modport_item: Box<ModportItem>,
    pub modport_list_list: Vec<ModportListList>,
    pub modport_list_opt: Option<Box<ModportListOpt>>,
}

///
/// Type derived for non-terminal ModportListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportListList {
    pub comma: Box<Comma>,
    pub modport_item: Box<ModportItem>,
}

///
/// Type derived for non-terminal ModportListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal ModportTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportTerm {
    pub modport_term: crate::veryl_token::Token, /* \bmodport\b */
}

///
/// Type derived for non-terminal ModportToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModportToken {
    pub modport_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Module
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Module {
    pub module_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ModuleDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
    pub module: Box<Module>,
    pub identifier: Box<Identifier>,
    pub module_declaration_opt: Option<Box<ModuleDeclarationOpt>>,
    pub module_declaration_opt0: Option<Box<ModuleDeclarationOpt0>>,
    pub l_brace: Box<LBrace>,
    pub module_declaration_list: Vec<ModuleDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ModuleDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleDeclarationList {
    pub module_item: Box<ModuleItem>,
}

///
/// Type derived for non-terminal ModuleDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleDeclarationOpt {
    pub with_parameter: Box<WithParameter>,
}

///
/// Type derived for non-terminal ModuleDeclarationOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleDeclarationOpt0 {
    pub port_declaration: Box<PortDeclaration>,
}

///
/// Type derived for non-terminal ModuleForDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleForDeclaration {
    pub r#for: Box<For>,
    pub identifier: Box<Identifier>,
    pub r#in: Box<In>,
    pub expression: Box<Expression>,
    pub dot_dot: Box<DotDot>,
    pub expression0: Box<Expression>,
    pub module_for_declaration_opt: Option<Box<ModuleForDeclarationOpt>>,
    pub colon: Box<Colon>,
    pub identifier0: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub module_for_declaration_list: Vec<ModuleForDeclarationList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ModuleForDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleForDeclarationList {
    pub module_item: Box<ModuleItem>,
}

///
/// Type derived for non-terminal ModuleForDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleForDeclarationOpt {
    pub step: Box<Step>,
    pub assignment_operator: Box<AssignmentOperator>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal ModuleIfDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclaration {
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub module_if_declaration_list: Vec<ModuleIfDeclarationList>,
    pub r_brace: Box<RBrace>,
    pub module_if_declaration_list0: Vec<ModuleIfDeclarationList0>,
    pub module_if_declaration_opt0: Option<Box<ModuleIfDeclarationOpt0>>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationList {
    pub module_item: Box<ModuleItem>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationList0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationList0 {
    pub r#else: Box<Else>,
    pub r#if: Box<If>,
    pub expression: Box<Expression>,
    pub module_if_declaration_opt: Option<Box<ModuleIfDeclarationOpt>>,
    pub l_brace: Box<LBrace>,
    pub module_if_declaration_list0_list: Vec<ModuleIfDeclarationList0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationList0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationList0List {
    pub module_item: Box<ModuleItem>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationOpt {
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationOpt0
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationOpt0 {
    pub r#else: Box<Else>,
    pub module_if_declaration_opt1: Option<Box<ModuleIfDeclarationOpt1>>,
    pub l_brace: Box<LBrace>,
    pub module_if_declaration_opt0_list: Vec<ModuleIfDeclarationOpt0List>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationOpt0List
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationOpt0List {
    pub module_item: Box<ModuleItem>,
}

///
/// Type derived for non-terminal ModuleIfDeclarationOpt1
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleIfDeclarationOpt1 {
    pub colon: Box<Colon>,
    pub identifier: Box<Identifier>,
}

///
/// Type derived for non-terminal ModuleItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ModuleItem {
    LetDeclaration(ModuleItemLetDeclaration),
    ParameterDeclaration(ModuleItemParameterDeclaration),
    LocalparamDeclaration(ModuleItemLocalparamDeclaration),
    AlwaysFfDeclaration(ModuleItemAlwaysFfDeclaration),
    AlwaysCombDeclaration(ModuleItemAlwaysCombDeclaration),
    AssignDeclaration(ModuleItemAssignDeclaration),
    FunctionDeclaration(ModuleItemFunctionDeclaration),
    ModuleIfDeclaration(ModuleItemModuleIfDeclaration),
    ModuleForDeclaration(ModuleItemModuleForDeclaration),
    EnumDeclaration(ModuleItemEnumDeclaration),
    StructDeclaration(ModuleItemStructDeclaration),
}

///
/// Type derived for non-terminal ModuleTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleTerm {
    pub module_term: crate::veryl_token::Token, /* \bmodule\b */
}

///
/// Type derived for non-terminal ModuleToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleToken {
    pub module_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Negedge
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Negedge {
    pub negedge_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal NegedgeTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NegedgeTerm {
    pub negedge_term: crate::veryl_token::Token, /* \bnegedge\b */
}

///
/// Type derived for non-terminal NegedgeToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NegedgeToken {
    pub negedge_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Number
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Number {
    IntegralNumber(NumberIntegralNumber),
    RealNumber(NumberRealNumber),
}

///
/// Type derived for non-terminal Operator01
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator01 {
    pub operator01_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal Operator01Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator01Term {
    pub operator01_term: crate::veryl_token::Token, /* \|\| */
}

///
/// Type derived for non-terminal Operator01Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator01Token {
    pub operator01_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator02Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator02Term {
    pub operator02_term: crate::veryl_token::Token, /* && */
}

///
/// Type derived for non-terminal Operator02Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator02Token {
    pub operator02_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator03Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator03Term {
    pub operator03_term: crate::veryl_token::Token, /* \| */
}

///
/// Type derived for non-terminal Operator03Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator03Token {
    pub operator03_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator04Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator04Term {
    pub operator04_term: crate::veryl_token::Token, /* \^~|\^|~\^ */
}

///
/// Type derived for non-terminal Operator04Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator04Token {
    pub operator04_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator05Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator05Term {
    pub operator05_term: crate::veryl_token::Token, /* & */
}

///
/// Type derived for non-terminal Operator05Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator05Token {
    pub operator05_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator06Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator06Term {
    pub operator06_term: crate::veryl_token::Token, /* ===|==\?|!==|!=\?|==|!= */
}

///
/// Type derived for non-terminal Operator06Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator06Token {
    pub operator06_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator07Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator07Term {
    pub operator07_term: crate::veryl_token::Token, /* <=|>=|<|> */
}

///
/// Type derived for non-terminal Operator07Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator07Token {
    pub operator07_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator08Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator08Term {
    pub operator08_term: crate::veryl_token::Token, /* <<<|>>>|<<|>> */
}

///
/// Type derived for non-terminal Operator08Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator08Token {
    pub operator08_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator09Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator09Term {
    pub operator09_term: crate::veryl_token::Token, /* \+|- */
}

///
/// Type derived for non-terminal Operator09Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator09Token {
    pub operator09_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator10Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator10Term {
    pub operator10_term: crate::veryl_token::Token, /* \*|/|% */
}

///
/// Type derived for non-terminal Operator10Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator10Token {
    pub operator10_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
/// Type derived for non-terminal Operator11Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator11Term {
    pub operator11_term: crate::veryl_token::Token, /* \*\* */
}

///
/// Type derived for non-terminal Operator11Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operator11Token {
    pub operator11_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Output
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Output {
    pub output_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal OutputTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OutputTerm {
    pub output_term: crate::veryl_token::Token, /* \boutput\b */
}

///
/// Type derived for non-terminal OutputToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OutputToken {
    pub output_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Parameter
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parameter {
    pub parameter_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ParameterDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParameterDeclaration {
    pub parameter: Box<Parameter>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal ParameterTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParameterTerm {
    pub parameter_term: crate::veryl_token::Token, /* \bparameter\b */
}

///
/// Type derived for non-terminal ParameterToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParameterToken {
    pub parameter_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal PlusColon
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlusColon {
    pub plus_colon_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal PlusColonTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlusColonTerm {
    pub plus_colon_term: crate::veryl_token::Token, /* +: */
}

///
/// Type derived for non-terminal PlusColonToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlusColonToken {
    pub plus_colon_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal PortDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclaration {
    pub l_paren: Box<LParen>,
    pub port_declaration_opt: Option<Box<PortDeclarationOpt>>,
    pub r_paren: Box<RParen>,
}

///
/// Type derived for non-terminal PortDeclarationItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclarationItem {
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub direction: Box<Direction>,
    pub r#type: Box<Type>,
}

///
/// Type derived for non-terminal PortDeclarationList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclarationList {
    pub port_declaration_item: Box<PortDeclarationItem>,
    pub port_declaration_list_list: Vec<PortDeclarationListList>,
    pub port_declaration_list_opt: Option<Box<PortDeclarationListOpt>>,
}

///
/// Type derived for non-terminal PortDeclarationListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclarationListList {
    pub comma: Box<Comma>,
    pub port_declaration_item: Box<PortDeclarationItem>,
}

///
/// Type derived for non-terminal PortDeclarationListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclarationListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal PortDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PortDeclarationOpt {
    pub port_declaration_list: Box<PortDeclarationList>,
}

///
/// Type derived for non-terminal Posedge
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Posedge {
    pub posedge_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal PosedgeTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PosedgeTerm {
    pub posedge_term: crate::veryl_token::Token, /* \bposedge\b */
}

///
/// Type derived for non-terminal PosedgeToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PosedgeToken {
    pub posedge_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal RBrace
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBrace {
    pub r_brace_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal RBraceTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBraceTerm {
    pub r_brace_term: crate::veryl_token::Token, /* } */
}

///
/// Type derived for non-terminal RBraceToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBraceToken {
    pub r_brace_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal RBracket
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBracket {
    pub r_bracket_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal RBracketTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBracketTerm {
    pub r_bracket_term: crate::veryl_token::Token, /* ] */
}

///
/// Type derived for non-terminal RBracketToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RBracketToken {
    pub r_bracket_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal RParen
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RParen {
    pub r_paren_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal RParenTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RParenTerm {
    pub r_paren_term: crate::veryl_token::Token, /* ) */
}

///
/// Type derived for non-terminal RParenToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RParenToken {
    pub r_paren_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
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
pub enum RangeOperator {
    Colon(RangeOperatorColon),
    PlusColon(RangeOperatorPlusColon),
    MinusColon(RangeOperatorMinusColon),
    Step(RangeOperatorStep),
}

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
/// Type derived for non-terminal RealNumber
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RealNumber {
    FixedPoint(RealNumberFixedPoint),
    Exponent(RealNumberExponent),
}

///
/// Type derived for non-terminal Ref
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Ref {
    pub ref_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal RefTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RefTerm {
    pub ref_term: crate::veryl_token::Token, /* \bref\b */
}

///
/// Type derived for non-terminal RefToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RefToken {
    pub ref_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Return
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Return {
    pub return_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal ReturnStatement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub r#return: Box<Return>,
    pub expression: Box<Expression>,
    pub semicolon: Box<Semicolon>,
}

///
/// Type derived for non-terminal ReturnTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReturnTerm {
    pub return_term: crate::veryl_token::Token, /* \breturn\b */
}

///
/// Type derived for non-terminal ReturnToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReturnToken {
    pub return_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Semicolon
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Semicolon {
    pub semicolon_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal SemicolonTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SemicolonTerm {
    pub semicolon_term: crate::veryl_token::Token, /* ; */
}

///
/// Type derived for non-terminal SemicolonToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SemicolonToken {
    pub semicolon_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Start
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Start {
    pub start_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal StartToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StartToken {
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Statement
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    AssignmentStatement(StatementAssignmentStatement),
    IfStatement(StatementIfStatement),
    IfResetStatement(StatementIfResetStatement),
    ReturnStatement(StatementReturnStatement),
    ForStatement(StatementForStatement),
}

///
/// Type derived for non-terminal Step
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Step {
    pub step_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal StepTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StepTerm {
    pub step_term: crate::veryl_token::Token, /* \bstep\b */
}

///
/// Type derived for non-terminal StepToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StepToken {
    pub step_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Struct
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Struct {
    pub struct_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal StructDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructDeclaration {
    pub r#struct: Box<Struct>,
    pub identifier: Box<Identifier>,
    pub l_brace: Box<LBrace>,
    pub struct_list: Box<StructList>,
    pub r_brace: Box<RBrace>,
}

///
/// Type derived for non-terminal StructItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructItem {
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
}

///
/// Type derived for non-terminal StructList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructList {
    pub struct_item: Box<StructItem>,
    pub struct_list_list: Vec<StructListList>,
    pub struct_list_opt: Option<Box<StructListOpt>>,
}

///
/// Type derived for non-terminal StructListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructListList {
    pub comma: Box<Comma>,
    pub struct_item: Box<StructItem>,
}

///
/// Type derived for non-terminal StructListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal StructTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructTerm {
    pub struct_term: crate::veryl_token::Token, /* \bstruct\b */
}

///
/// Type derived for non-terminal StructToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructToken {
    pub struct_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal SyncHigh
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncHigh {
    pub sync_high_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal SyncHighTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncHighTerm {
    pub sync_high_term: crate::veryl_token::Token, /* \bsync_high\b */
}

///
/// Type derived for non-terminal SyncHighToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncHighToken {
    pub sync_high_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal SyncLow
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncLow {
    pub sync_low_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal SyncLowTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncLowTerm {
    pub sync_low_term: crate::veryl_token::Token, /* \bsync_low\b */
}

///
/// Type derived for non-terminal SyncLowToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyncLowToken {
    pub sync_low_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal Type
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Type {
    pub type_group: Box<TypeGroup>,
    pub type_list: Vec<TypeList>,
}

///
/// Type derived for non-terminal TypeGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TypeGroup {
    BuiltinType(TypeGroupBuiltinType),
    Identifier(TypeGroupIdentifier),
}

///
/// Type derived for non-terminal TypeList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeList {
    pub width: Box<Width>,
}

///
/// Type derived for non-terminal U32
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U32 {
    pub u32_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal U32Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U32Term {
    pub u32_term: crate::veryl_token::Token, /* \bu32\b */
}

///
/// Type derived for non-terminal U32Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U32Token {
    pub u32_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal U64
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U64 {
    pub u64_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal U64Term
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U64Term {
    pub u64_term: crate::veryl_token::Token, /* \bu64\b */
}

///
/// Type derived for non-terminal U64Token
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct U64Token {
    pub u64_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal UnaryOperator
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnaryOperator {
    pub unary_operator_token: crate::veryl_token::VerylToken,
}

///
/// Type derived for non-terminal UnaryOperatorTerm
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnaryOperatorTerm {
    pub unary_operator_term: crate::veryl_token::Token, /* ~&|~\||!|~ */
}

///
/// Type derived for non-terminal UnaryOperatorToken
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnaryOperatorToken {
    pub unary_operator_term: crate::veryl_token::Token,
    pub comments: Box<Comments>,
}

///
/// Type derived for non-terminal VariableDeclaration
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub r#type: Box<Type>,
    pub variable_declaration_opt: Option<Box<VariableDeclarationOpt>>,
}

///
/// Type derived for non-terminal VariableDeclarationOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariableDeclarationOpt {
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal Veryl
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Veryl {
    pub start: Box<Start>,
    pub veryl_list: Vec<VerylList>,
}

///
/// Type derived for non-terminal VerylList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VerylList {
    pub description: Box<Description>,
}

///
/// Type derived for non-terminal Width
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Width {
    pub l_bracket: Box<LBracket>,
    pub expression: Box<Expression>,
    pub r_bracket: Box<RBracket>,
}

///
/// Type derived for non-terminal WithParameter
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameter {
    pub hash: Box<Hash>,
    pub l_paren: Box<LParen>,
    pub with_parameter_opt: Option<Box<WithParameterOpt>>,
    pub r_paren: Box<RParen>,
}

///
/// Type derived for non-terminal WithParameterItem
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterItem {
    pub with_parameter_item_group: Box<WithParameterItemGroup>,
    pub identifier: Box<Identifier>,
    pub colon: Box<Colon>,
    pub r#type: Box<Type>,
    pub equ: Box<Equ>,
    pub expression: Box<Expression>,
}

///
/// Type derived for non-terminal WithParameterItemGroup
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WithParameterItemGroup {
    Parameter(WithParameterItemGroupParameter),
    Localparam(WithParameterItemGroupLocalparam),
}

///
/// Type derived for non-terminal WithParameterList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterList {
    pub with_parameter_item: Box<WithParameterItem>,
    pub with_parameter_list_list: Vec<WithParameterListList>,
    pub with_parameter_list_opt: Option<Box<WithParameterListOpt>>,
}

///
/// Type derived for non-terminal WithParameterListList
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterListList {
    pub comma: Box<Comma>,
    pub with_parameter_item: Box<WithParameterItem>,
}

///
/// Type derived for non-terminal WithParameterListOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterListOpt {
    pub comma: Box<Comma>,
}

///
/// Type derived for non-terminal WithParameterOpt
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithParameterOpt {
    pub with_parameter_list: Box<WithParameterList>,
}
