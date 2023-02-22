use crate::veryl_grammar_trait::*;
use crate::veryl_token::VerylToken;

pub trait VerylWalker {
    /// Semantic action for non-terminal 'VerylToken'
    fn veryl_token(&mut self, _arg: &VerylToken) {}

    /// Semantic action for non-terminal 'Start'
    fn start(&mut self, arg: &Start) {
        self.veryl_token(&arg.start_token);
    }

    /// Semantic action for non-terminal 'Exponent'
    fn exponent(&mut self, arg: &Exponent) {
        self.veryl_token(&arg.exponent_token);
    }

    /// Semantic action for non-terminal 'FixedPoint'
    fn fixed_point(&mut self, arg: &FixedPoint) {
        self.veryl_token(&arg.fixed_point_token);
    }

    /// Semantic action for non-terminal 'Based'
    fn based(&mut self, arg: &Based) {
        self.veryl_token(&arg.based_token);
    }

    /// Semantic action for non-terminal 'BaseLess'
    fn base_less(&mut self, arg: &BaseLess) {
        self.veryl_token(&arg.base_less_token);
    }

    /// Semantic action for non-terminal 'AllBit'
    fn all_bit(&mut self, arg: &AllBit) {
        self.veryl_token(&arg.all_bit_token);
    }

    /// Semantic action for non-terminal 'AssignmentOperator'
    fn assignment_operator(&mut self, arg: &AssignmentOperator) {
        self.veryl_token(&arg.assignment_operator_token);
    }

    /// Semantic action for non-terminal 'Operator01'
    fn operator01(&mut self, arg: &Operator01) {
        self.veryl_token(&arg.operator01_token);
    }

    /// Semantic action for non-terminal 'Operator02'
    fn operator02(&mut self, arg: &Operator02) {
        self.veryl_token(&arg.operator02_token);
    }

    /// Semantic action for non-terminal 'Operator03'
    fn operator03(&mut self, arg: &Operator03) {
        self.veryl_token(&arg.operator03_token);
    }

    /// Semantic action for non-terminal 'Operator04'
    fn operator04(&mut self, arg: &Operator04) {
        self.veryl_token(&arg.operator04_token);
    }

    /// Semantic action for non-terminal 'Operator05'
    fn operator05(&mut self, arg: &Operator05) {
        self.veryl_token(&arg.operator05_token);
    }

    /// Semantic action for non-terminal 'Operator06'
    fn operator06(&mut self, arg: &Operator06) {
        self.veryl_token(&arg.operator06_token);
    }

    /// Semantic action for non-terminal 'Operator07'
    fn operator07(&mut self, arg: &Operator07) {
        self.veryl_token(&arg.operator07_token);
    }

    /// Semantic action for non-terminal 'Operator08'
    fn operator08(&mut self, arg: &Operator08) {
        self.veryl_token(&arg.operator08_token);
    }

    /// Semantic action for non-terminal 'Operator09'
    fn operator09(&mut self, arg: &Operator09) {
        self.veryl_token(&arg.operator09_token);
    }

    /// Semantic action for non-terminal 'Operator10'
    fn operator10(&mut self, arg: &Operator10) {
        self.veryl_token(&arg.operator10_token);
    }

    /// Semantic action for non-terminal 'Operator11'
    fn operator11(&mut self, arg: &Operator11) {
        self.veryl_token(&arg.operator11_token);
    }

    /// Semantic action for non-terminal 'UnaryOperator'
    fn unary_operator(&mut self, arg: &UnaryOperator) {
        self.veryl_token(&arg.unary_operator_token);
    }

    /// Semantic action for non-terminal 'Colon'
    fn colon(&mut self, arg: &Colon) {
        self.veryl_token(&arg.colon_token);
    }

    /// Semantic action for non-terminal 'Comma'
    fn comma(&mut self, arg: &Comma) {
        self.veryl_token(&arg.comma_token);
    }

    /// Semantic action for non-terminal 'Dollar'
    fn dollar(&mut self, arg: &Dollar) {
        self.veryl_token(&arg.dollar_token);
    }

    /// Semantic action for non-terminal 'DotDot'
    fn dot_dot(&mut self, arg: &DotDot) {
        self.veryl_token(&arg.dot_dot_token);
    }

    /// Semantic action for non-terminal 'Dot'
    fn dot(&mut self, arg: &Dot) {
        self.veryl_token(&arg.dot_token);
    }

    /// Semantic action for non-terminal 'Equ'
    fn equ(&mut self, arg: &Equ) {
        self.veryl_token(&arg.equ_token);
    }

    /// Semantic action for non-terminal 'Hash'
    fn hash(&mut self, arg: &Hash) {
        self.veryl_token(&arg.hash_token);
    }

    /// Semantic action for non-terminal 'LBrace'
    fn l_brace(&mut self, arg: &LBrace) {
        self.veryl_token(&arg.l_brace_token);
    }

    /// Semantic action for non-terminal 'LBracket'
    fn l_bracket(&mut self, arg: &LBracket) {
        self.veryl_token(&arg.l_bracket_token);
    }

    /// Semantic action for non-terminal 'LParen'
    fn l_paren(&mut self, arg: &LParen) {
        self.veryl_token(&arg.l_paren_token);
    }

    /// Semantic action for non-terminal 'MinusColon'
    fn minus_colon(&mut self, arg: &MinusColon) {
        self.veryl_token(&arg.minus_colon_token);
    }

    /// Semantic action for non-terminal 'MinusGT'
    fn minus_g_t(&mut self, arg: &MinusGT) {
        self.veryl_token(&arg.minus_g_t_token);
    }

    /// Semantic action for non-terminal 'PlusColon'
    fn plus_colon(&mut self, arg: &PlusColon) {
        self.veryl_token(&arg.plus_colon_token);
    }

    /// Semantic action for non-terminal 'RBrace'
    fn r_brace(&mut self, arg: &RBrace) {
        self.veryl_token(&arg.r_brace_token);
    }

    /// Semantic action for non-terminal 'RBracket'
    fn r_bracket(&mut self, arg: &RBracket) {
        self.veryl_token(&arg.r_bracket_token);
    }

    /// Semantic action for non-terminal 'RParen'
    fn r_paren(&mut self, arg: &RParen) {
        self.veryl_token(&arg.r_paren_token);
    }

    /// Semantic action for non-terminal 'Semicolon'
    fn semicolon(&mut self, arg: &Semicolon) {
        self.veryl_token(&arg.semicolon_token);
    }

    /// Semantic action for non-terminal 'AlwaysComb'
    fn always_comb(&mut self, arg: &AlwaysComb) {
        self.veryl_token(&arg.always_comb_token);
    }

    /// Semantic action for non-terminal 'AlwaysFf'
    fn always_ff(&mut self, arg: &AlwaysFf) {
        self.veryl_token(&arg.always_ff_token);
    }

    /// Semantic action for non-terminal 'Assign'
    fn assign(&mut self, arg: &Assign) {
        self.veryl_token(&arg.assign_token);
    }

    /// Semantic action for non-terminal 'AsyncHigh'
    fn async_high(&mut self, arg: &AsyncHigh) {
        self.veryl_token(&arg.async_high_token);
    }

    /// Semantic action for non-terminal 'AsyncLow'
    fn async_low(&mut self, arg: &AsyncLow) {
        self.veryl_token(&arg.async_low_token);
    }

    /// Semantic action for non-terminal 'Bit'
    fn bit(&mut self, arg: &Bit) {
        self.veryl_token(&arg.bit_token);
    }

    /// Semantic action for non-terminal 'Else'
    fn r#else(&mut self, arg: &Else) {
        self.veryl_token(&arg.else_token);
    }

    /// Semantic action for non-terminal 'Enum'
    fn r#enum(&mut self, arg: &Enum) {
        self.veryl_token(&arg.enum_token);
    }

    /// Semantic action for non-terminal 'F32'
    fn f32(&mut self, arg: &F32) {
        self.veryl_token(&arg.f32_token);
    }

    /// Semantic action for non-terminal 'F64'
    fn f64(&mut self, arg: &F64) {
        self.veryl_token(&arg.f64_token);
    }

    /// Semantic action for non-terminal 'For'
    fn r#for(&mut self, arg: &For) {
        self.veryl_token(&arg.for_token);
    }

    /// Semantic action for non-terminal 'Function'
    fn function(&mut self, arg: &Function) {
        self.veryl_token(&arg.function_token);
    }

    /// Semantic action for non-terminal 'I32'
    fn i32(&mut self, arg: &I32) {
        self.veryl_token(&arg.i32_token);
    }

    /// Semantic action for non-terminal 'I64'
    fn i64(&mut self, arg: &I64) {
        self.veryl_token(&arg.i64_token);
    }

    /// Semantic action for non-terminal 'If'
    fn r#if(&mut self, arg: &If) {
        self.veryl_token(&arg.if_token);
    }

    /// Semantic action for non-terminal 'IfReset'
    fn if_reset(&mut self, arg: &IfReset) {
        self.veryl_token(&arg.if_reset_token);
    }

    /// Semantic action for non-terminal 'In'
    fn r#in(&mut self, arg: &In) {
        self.veryl_token(&arg.in_token);
    }

    /// Semantic action for non-terminal 'Inout'
    fn inout(&mut self, arg: &Inout) {
        self.veryl_token(&arg.inout_token);
    }

    /// Semantic action for non-terminal 'Input'
    fn input(&mut self, arg: &Input) {
        self.veryl_token(&arg.input_token);
    }

    /// Semantic action for non-terminal 'Inst'
    fn inst(&mut self, arg: &Inst) {
        self.veryl_token(&arg.inst_token);
    }

    /// Semantic action for non-terminal 'Interface'
    fn interface(&mut self, arg: &Interface) {
        self.veryl_token(&arg.interface_token);
    }

    /// Semantic action for non-terminal 'Let'
    fn r#let(&mut self, arg: &Let) {
        self.veryl_token(&arg.let_token);
    }

    /// Semantic action for non-terminal 'Localparam'
    fn localparam(&mut self, arg: &Localparam) {
        self.veryl_token(&arg.localparam_token);
    }

    /// Semantic action for non-terminal 'Logic'
    fn logic(&mut self, arg: &Logic) {
        self.veryl_token(&arg.logic_token);
    }

    /// Semantic action for non-terminal 'Modport'
    fn modport(&mut self, arg: &Modport) {
        self.veryl_token(&arg.modport_token);
    }

    /// Semantic action for non-terminal 'Module'
    fn module(&mut self, arg: &Module) {
        self.veryl_token(&arg.module_token);
    }

    /// Semantic action for non-terminal 'Negedge'
    fn negedge(&mut self, arg: &Negedge) {
        self.veryl_token(&arg.negedge_token);
    }

    /// Semantic action for non-terminal 'Output'
    fn output(&mut self, arg: &Output) {
        self.veryl_token(&arg.output_token);
    }

    /// Semantic action for non-terminal 'Parameter'
    fn parameter(&mut self, arg: &Parameter) {
        self.veryl_token(&arg.parameter_token);
    }

    /// Semantic action for non-terminal 'Posedge'
    fn posedge(&mut self, arg: &Posedge) {
        self.veryl_token(&arg.posedge_token);
    }

    /// Semantic action for non-terminal 'Ref'
    fn r#ref(&mut self, arg: &Ref) {
        self.veryl_token(&arg.ref_token);
    }

    /// Semantic action for non-terminal 'Return'
    fn r#return(&mut self, arg: &Return) {
        self.veryl_token(&arg.return_token);
    }

    /// Semantic action for non-terminal 'Step'
    fn step(&mut self, arg: &Step) {
        self.veryl_token(&arg.step_token);
    }

    /// Semantic action for non-terminal 'Struct'
    fn r#struct(&mut self, arg: &Struct) {
        self.veryl_token(&arg.struct_token);
    }

    /// Semantic action for non-terminal 'SyncHigh'
    fn sync_high(&mut self, arg: &SyncHigh) {
        self.veryl_token(&arg.sync_high_token);
    }

    /// Semantic action for non-terminal 'SyncLow'
    fn sync_low(&mut self, arg: &SyncLow) {
        self.veryl_token(&arg.sync_low_token);
    }

    /// Semantic action for non-terminal 'U32'
    fn u32(&mut self, arg: &U32) {
        self.veryl_token(&arg.u32_token);
    }

    /// Semantic action for non-terminal 'U64'
    fn u64(&mut self, arg: &U64) {
        self.veryl_token(&arg.u64_token);
    }

    /// Semantic action for non-terminal 'Identifier'
    fn identifier(&mut self, arg: &Identifier) {
        self.veryl_token(&arg.identifier_token);
    }

    /// Semantic action for non-terminal 'Number'
    fn number(&mut self, arg: &Number) {
        match arg {
            Number::IntegralNumber(x) => self.integral_number(&x.integral_number),
            Number::RealNumber(x) => self.real_number(&x.real_number),
        };
    }

    /// Semantic action for non-terminal 'IntegralNumber'
    fn integral_number(&mut self, arg: &IntegralNumber) {
        match arg {
            IntegralNumber::Based(x) => self.based(&x.based),
            IntegralNumber::BaseLess(x) => self.base_less(&x.base_less),
            IntegralNumber::AllBit(x) => self.all_bit(&x.all_bit),
        };
    }

    /// Semantic action for non-terminal 'RealNumber'
    fn real_number(&mut self, arg: &RealNumber) {
        match arg {
            RealNumber::FixedPoint(x) => self.fixed_point(&x.fixed_point),
            RealNumber::Exponent(x) => self.exponent(&x.exponent),
        };
    }

    /// Semantic action for non-terminal 'HierarchicalIdentifier'
    fn hierarchical_identifier(&mut self, arg: &HierarchicalIdentifier) {
        self.identifier(&arg.identifier);
        for x in &arg.hierarchical_identifier_list {
            self.range(&x.range);
        }
        for x in &arg.hierarchical_identifier_list0 {
            self.dot(&x.dot);
            self.identifier(&x.identifier);
            for x in &x.hierarchical_identifier_list0_list {
                self.range(&x.range);
            }
        }
    }

    /// Semantic action for non-terminal 'Expression'
    fn expression(&mut self, arg: &Expression) {
        self.expression01(&arg.expression01);
        for x in &arg.expression_list {
            self.operator01(&x.operator01);
            self.expression01(&x.expression01);
        }
    }

    /// Semantic action for non-terminal 'Expression01'
    fn expression01(&mut self, arg: &Expression01) {
        self.expression02(&arg.expression02);
        for x in &arg.expression01_list {
            self.operator02(&x.operator02);
            self.expression02(&x.expression02);
        }
    }

    /// Semantic action for non-terminal 'Expression02'
    fn expression02(&mut self, arg: &Expression02) {
        self.expression03(&arg.expression03);
        for x in &arg.expression02_list {
            self.operator03(&x.operator03);
            self.expression03(&x.expression03);
        }
    }

    /// Semantic action for non-terminal 'Expression03'
    fn expression03(&mut self, arg: &Expression03) {
        self.expression04(&arg.expression04);
        for x in &arg.expression03_list {
            self.operator04(&x.operator04);
            self.expression04(&x.expression04);
        }
    }

    /// Semantic action for non-terminal 'Expression04'
    fn expression04(&mut self, arg: &Expression04) {
        self.expression05(&arg.expression05);
        for x in &arg.expression04_list {
            self.operator05(&x.operator05);
            self.expression05(&x.expression05);
        }
    }

    /// Semantic action for non-terminal 'Expression05'
    fn expression05(&mut self, arg: &Expression05) {
        self.expression06(&arg.expression06);
        for x in &arg.expression05_list {
            self.operator06(&x.operator06);
            self.expression06(&x.expression06);
        }
    }

    /// Semantic action for non-terminal 'Expression06'
    fn expression06(&mut self, arg: &Expression06) {
        self.expression07(&arg.expression07);
        for x in &arg.expression06_list {
            self.operator07(&x.operator07);
            self.expression07(&x.expression07);
        }
    }

    /// Semantic action for non-terminal 'Expression07'
    fn expression07(&mut self, arg: &Expression07) {
        self.expression08(&arg.expression08);
        for x in &arg.expression07_list {
            self.operator08(&x.operator08);
            self.expression08(&x.expression08);
        }
    }

    /// Semantic action for non-terminal 'Expression08'
    fn expression08(&mut self, arg: &Expression08) {
        self.expression09(&arg.expression09);
        for x in &arg.expression08_list {
            self.operator09(&x.operator09);
            self.expression09(&x.expression09);
        }
    }

    /// Semantic action for non-terminal 'Expression09'
    fn expression09(&mut self, arg: &Expression09) {
        self.expression10(&arg.expression10);
        for x in &arg.expression09_list {
            self.operator10(&x.operator10);
            self.expression10(&x.expression10);
        }
    }

    /// Semantic action for non-terminal 'Expression10'
    fn expression10(&mut self, arg: &Expression10) {
        self.expression11(&arg.expression11);
        for x in &arg.expression10_list {
            self.operator11(&x.operator11);
            self.expression11(&x.expression11);
        }
    }

    /// Semantic action for non-terminal 'Expression11'
    fn expression11(&mut self, arg: &Expression11) {
        if let Some(ref x) = arg.expression11_opt {
            match &*x.expression11_opt_group {
                Expression11OptGroup::UnaryOperator(x) => self.unary_operator(&x.unary_operator),
                Expression11OptGroup::Operator03(x) => self.operator03(&x.operator03),
                Expression11OptGroup::Operator04(x) => self.operator04(&x.operator04),
                Expression11OptGroup::Operator05(x) => self.operator05(&x.operator05),
                Expression11OptGroup::Operator09(x) => self.operator09(&x.operator09),
            }
        }
        self.factor(&arg.factor);
    }

    /// Semantic action for non-terminal 'Factor'
    fn factor(&mut self, arg: &Factor) {
        match arg {
            Factor::Number(x) => self.number(&x.number),
            Factor::FactorOptHierarchicalIdentifierFactorOpt0(x) => {
                if let Some(ref x) = x.factor_opt {
                    self.dollar(&x.dollar);
                }
                self.hierarchical_identifier(&x.hierarchical_identifier);
                if let Some(ref x) = x.factor_opt0 {
                    self.l_paren(&x.l_paren);
                    if let Some(ref x) = x.factor_opt1 {
                        self.function_call_arg(&x.function_call_arg);
                    }
                    self.r_paren(&x.r_paren);
                }
            }
            Factor::LParenExpressionRParen(x) => {
                self.l_paren(&x.l_paren);
                self.expression(&x.expression);
                self.r_paren(&x.r_paren);
            }
        }
    }

    /// Semantic action for non-terminal 'FunctionCallArg'
    fn function_call_arg(&mut self, arg: &FunctionCallArg) {
        self.expression(&arg.expression);
        for x in &arg.function_call_arg_list {
            self.comma(&x.comma);
            self.expression(&x.expression);
        }
        if let Some(ref x) = arg.function_call_arg_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'Range'
    fn range(&mut self, arg: &Range) {
        self.l_bracket(&arg.l_bracket);
        self.expression(&arg.expression);
        if let Some(ref x) = arg.range_opt {
            self.range_operator(&x.range_operator);
            self.expression(&x.expression);
        }
        self.r_bracket(&arg.r_bracket);
    }

    /// Semantic action for non-terminal 'RangeOperator'
    fn range_operator(&mut self, arg: &RangeOperator) {
        match arg {
            RangeOperator::Colon(x) => self.colon(&x.colon),
            RangeOperator::PlusColon(x) => self.plus_colon(&x.plus_colon),
            RangeOperator::MinusColon(x) => self.minus_colon(&x.minus_colon),
            RangeOperator::Step(x) => self.step(&x.step),
        }
    }

    /// Semantic action for non-terminal 'Width'
    fn width(&mut self, arg: &Width) {
        self.l_bracket(&arg.l_bracket);
        self.expression(&arg.expression);
        self.r_bracket(&arg.r_bracket);
    }

    /// Semantic action for non-terminal 'BuiltinType'
    fn builtin_type(&mut self, arg: &BuiltinType) {
        match arg {
            BuiltinType::Logic(x) => self.logic(&x.logic),
            BuiltinType::Bit(x) => self.bit(&x.bit),
            BuiltinType::U32(x) => self.u32(&x.u32),
            BuiltinType::U64(x) => self.u64(&x.u64),
            BuiltinType::I32(x) => self.i32(&x.i32),
            BuiltinType::I64(x) => self.i64(&x.i64),
            BuiltinType::F32(x) => self.f32(&x.f32),
            BuiltinType::F64(x) => self.f64(&x.f64),
        };
    }

    /// Semantic action for non-terminal 'Type'
    fn r#type(&mut self, arg: &Type) {
        match &*arg.type_group {
            TypeGroup::BuiltinType(x) => self.builtin_type(&x.builtin_type),
            TypeGroup::Identifier(x) => self.identifier(&x.identifier),
        };
        for x in &arg.type_list {
            self.width(&x.width);
        }
    }

    /// Semantic action for non-terminal 'Statement'
    fn statement(&mut self, arg: &Statement) {
        match arg {
            Statement::AssignmentStatement(x) => self.assignment_statement(&x.assignment_statement),
            Statement::IfStatement(x) => self.if_statement(&x.if_statement),
            Statement::IfResetStatement(x) => self.if_reset_statement(&x.if_reset_statement),
            Statement::ReturnStatement(x) => self.return_statement(&x.return_statement),
            Statement::ForStatement(x) => self.for_statement(&x.for_statement),
        };
    }

    /// Semantic action for non-terminal 'AssignmentStatement'
    fn assignment_statement(&mut self, arg: &AssignmentStatement) {
        self.hierarchical_identifier(&arg.hierarchical_identifier);
        match &*arg.assignment_statement_group {
            AssignmentStatementGroup::Equ(x) => self.equ(&x.equ),
            AssignmentStatementGroup::AssignmentOperator(x) => {
                self.assignment_operator(&x.assignment_operator)
            }
        }
        self.expression(&arg.expression);
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'IfStatement'
    fn if_statement(&mut self, arg: &IfStatement) {
        self.r#if(&arg.r#if);
        self.expression(&arg.expression);
        self.l_brace(&arg.l_brace);
        for x in &arg.if_statement_list {
            self.statement(&x.statement);
        }
        self.r_brace(&arg.r_brace);
        for x in &arg.if_statement_list0 {
            self.r#else(&x.r#else);
            self.r#if(&x.r#if);
            self.expression(&x.expression);
            self.l_brace(&x.l_brace);
            for x in &x.if_statement_list0_list {
                self.statement(&x.statement);
            }
            self.r_brace(&x.r_brace);
        }
        if let Some(ref x) = arg.if_statement_opt {
            self.r#else(&x.r#else);
            self.l_brace(&x.l_brace);
            for x in &x.if_statement_opt_list {
                self.statement(&x.statement);
            }
            self.r_brace(&x.r_brace);
        }
    }

    /// Semantic action for non-terminal 'IfResetStatement'
    fn if_reset_statement(&mut self, arg: &IfResetStatement) {
        self.if_reset(&arg.if_reset);
        self.l_brace(&arg.l_brace);
        for x in &arg.if_reset_statement_list {
            self.statement(&x.statement);
        }
        self.r_brace(&arg.r_brace);
        for x in &arg.if_reset_statement_list0 {
            self.r#else(&x.r#else);
            self.r#if(&x.r#if);
            self.expression(&x.expression);
            self.l_brace(&x.l_brace);
            for x in &x.if_reset_statement_list0_list {
                self.statement(&x.statement);
            }
            self.r_brace(&x.r_brace);
        }
        if let Some(ref x) = arg.if_reset_statement_opt {
            self.r#else(&x.r#else);
            self.l_brace(&x.l_brace);
            for x in &x.if_reset_statement_opt_list {
                self.statement(&x.statement);
            }
            self.r_brace(&x.r_brace);
        }
    }

    /// Semantic action for non-terminal 'ReturnStatement'
    fn return_statement(&mut self, arg: &ReturnStatement) {
        self.r#return(&arg.r#return);
        self.expression(&arg.expression);
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'ForStatement'
    fn for_statement(&mut self, arg: &ForStatement) {
        self.r#for(&arg.r#for);
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
        self.r#in(&arg.r#in);
        self.expression(&arg.expression);
        self.dot_dot(&arg.dot_dot);
        self.expression(&arg.expression0);
        if let Some(ref x) = arg.for_statement_opt {
            self.step(&x.step);
            self.assignment_operator(&x.assignment_operator);
            self.expression(&x.expression);
        }
        self.l_brace(&arg.l_brace);
        for x in &arg.for_statement_list {
            self.statement(&x.statement);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'LetDeclaration'
    fn let_declaration(&mut self, arg: &LetDeclaration) {
        self.r#let(&arg.r#let);
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        match &*arg.let_declaration_group {
            LetDeclarationGroup::VariableDeclaration(x) => {
                self.variable_declaration(&x.variable_declaration)
            }
            LetDeclarationGroup::InstanceDeclaration(x) => {
                self.instance_declaration(&x.instance_declaration)
            }
        }
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'VariableDeclaration'
    fn variable_declaration(&mut self, arg: &VariableDeclaration) {
        self.r#type(&arg.r#type);
        if let Some(ref x) = arg.variable_declaration_opt {
            self.equ(&x.equ);
            self.expression(&x.expression);
        }
    }

    /// Semantic action for non-terminal 'ParameterDeclaration'
    fn parameter_declaration(&mut self, arg: &ParameterDeclaration) {
        self.parameter(&arg.parameter);
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
        self.equ(&arg.equ);
        self.expression(&arg.expression);
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'LocalparamDeclaration'
    fn localparam_declaration(&mut self, arg: &LocalparamDeclaration) {
        self.localparam(&arg.localparam);
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
        self.equ(&arg.equ);
        self.expression(&arg.expression);
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'AlwaysFfDeclaration'
    fn always_ff_declaration(&mut self, arg: &AlwaysFfDeclaration) {
        self.always_ff(&arg.always_ff);
        self.l_paren(&arg.l_paren);
        self.always_ff_clock(&arg.always_ff_clock);
        if let Some(ref x) = arg.always_ff_declaration_opt {
            self.comma(&x.comma);
            self.always_ff_reset(&x.always_ff_reset);
        }
        self.r_paren(&arg.r_paren);
        self.l_brace(&arg.l_brace);
        for x in &arg.always_ff_declaration_list {
            self.statement(&x.statement);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'AlwaysFfClock'
    fn always_ff_clock(&mut self, arg: &AlwaysFfClock) {
        if let Some(ref x) = arg.always_ff_clock_opt {
            match &*x.always_ff_clock_opt_group {
                AlwaysFfClockOptGroup::Posedge(x) => self.posedge(&x.posedge),
                AlwaysFfClockOptGroup::Negedge(x) => self.negedge(&x.negedge),
            }
        }
        self.hierarchical_identifier(&arg.hierarchical_identifier);
    }

    /// Semantic action for non-terminal 'AlwaysFfReset'
    fn always_ff_reset(&mut self, arg: &AlwaysFfReset) {
        if let Some(ref x) = arg.always_ff_reset_opt {
            match &*x.always_ff_reset_opt_group {
                AlwaysFfResetOptGroup::AsyncLow(x) => self.async_low(&x.async_low),
                AlwaysFfResetOptGroup::AsyncHigh(x) => self.async_high(&x.async_high),
                AlwaysFfResetOptGroup::SyncLow(x) => self.sync_low(&x.sync_low),
                AlwaysFfResetOptGroup::SyncHigh(x) => self.sync_high(&x.sync_high),
            }
        }
        self.hierarchical_identifier(&arg.hierarchical_identifier);
    }

    /// Semantic action for non-terminal 'AlwaysCombDeclaration'
    fn always_comb_declaration(&mut self, arg: &AlwaysCombDeclaration) {
        self.always_comb(&arg.always_comb);
        self.l_brace(&arg.l_brace);
        for x in &arg.always_comb_declaration_list {
            self.statement(&x.statement);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'AssignDeclaration'
    fn assign_declaration(&mut self, arg: &AssignDeclaration) {
        self.assign(&arg.assign);
        self.hierarchical_identifier(&arg.hierarchical_identifier);
        self.equ(&arg.equ);
        self.expression(&arg.expression);
        self.semicolon(&arg.semicolon);
    }

    /// Semantic action for non-terminal 'ModportDeclaration'
    fn modport_declaration(&mut self, arg: &ModportDeclaration) {
        self.modport(&arg.modport);
        self.identifier(&arg.identifier);
        self.l_brace(&arg.l_brace);
        self.modport_list(&arg.modport_list);
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'ModportList'
    fn modport_list(&mut self, arg: &ModportList) {
        self.modport_item(&arg.modport_item);
        for x in &arg.modport_list_list {
            self.comma(&x.comma);
            self.modport_item(&x.modport_item);
        }
        if let Some(ref x) = arg.modport_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'ModportItem'
    fn modport_item(&mut self, arg: &ModportItem) {
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.direction(&arg.direction);
    }

    /// Semantic action for non-terminal 'EnumDeclaration'
    fn enum_declaration(&mut self, arg: &EnumDeclaration) {
        self.r#enum(&arg.r#enum);
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
        self.l_brace(&arg.l_brace);
        self.enum_list(&arg.enum_list);
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'EnumList'
    fn enum_list(&mut self, arg: &EnumList) {
        self.enum_item(&arg.enum_item);
        for x in &arg.enum_list_list {
            self.comma(&x.comma);
            self.enum_item(&x.enum_item);
        }
        if let Some(ref x) = arg.enum_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'EnumItem'
    fn enum_item(&mut self, arg: &EnumItem) {
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.enum_item_opt {
            self.equ(&x.equ);
            self.expression(&x.expression);
        }
    }

    /// Semantic action for non-terminal 'StructDeclaration'
    fn struct_declaration(&mut self, arg: &StructDeclaration) {
        self.r#struct(&arg.r#struct);
        self.identifier(&arg.identifier);
        self.l_brace(&arg.l_brace);
        self.struct_list(&arg.struct_list);
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'StructList'
    fn struct_list(&mut self, arg: &StructList) {
        self.struct_item(&arg.struct_item);
        for x in &arg.struct_list_list {
            self.comma(&x.comma);
            self.struct_item(&x.struct_item);
        }
        if let Some(ref x) = arg.struct_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'StructItem'
    fn struct_item(&mut self, arg: &StructItem) {
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
    }

    /// Semantic action for non-terminal 'InstanceDeclaration'
    fn instance_declaration(&mut self, arg: &InstanceDeclaration) {
        self.inst(&arg.inst);
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.instance_declaration_opt {
            self.width(&x.width);
        }
        if let Some(ref x) = arg.instance_declaration_opt0 {
            self.instance_parameter(&x.instance_parameter);
        }
        if let Some(ref x) = arg.instance_declaration_opt1 {
            self.l_brace(&x.l_brace);
            if let Some(ref x) = x.instance_declaration_opt2 {
                self.instance_port_list(&x.instance_port_list);
            }
            self.r_brace(&x.r_brace);
        }
    }

    /// Semantic action for non-terminal 'InstanceParameter'
    fn instance_parameter(&mut self, arg: &InstanceParameter) {
        self.hash(&arg.hash);
        self.l_paren(&arg.l_paren);
        if let Some(ref x) = arg.instance_parameter_opt {
            self.instance_parameter_list(&x.instance_parameter_list);
        }
        self.r_paren(&arg.r_paren);
    }

    /// Semantic action for non-terminal 'InstanceParameterList'
    fn instance_parameter_list(&mut self, arg: &InstanceParameterList) {
        self.instance_parameter_item(&arg.instance_parameter_item);
        for x in &arg.instance_parameter_list_list {
            self.comma(&x.comma);
            self.instance_parameter_item(&x.instance_parameter_item);
        }
        if let Some(ref x) = arg.instance_parameter_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'InstanceParameterItem'
    fn instance_parameter_item(&mut self, arg: &InstanceParameterItem) {
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.instance_parameter_item_opt {
            self.colon(&x.colon);
            self.expression(&x.expression);
        }
    }

    /// Semantic action for non-terminal 'InstancePortList'
    fn instance_port_list(&mut self, arg: &InstancePortList) {
        self.instance_port_item(&arg.instance_port_item);
        for x in &arg.instance_port_list_list {
            self.comma(&x.comma);
            self.instance_port_item(&x.instance_port_item);
        }
        if let Some(ref x) = arg.instance_port_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'InstancePortItem'
    fn instance_port_item(&mut self, arg: &InstancePortItem) {
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.instance_port_item_opt {
            self.colon(&x.colon);
            self.expression(&x.expression);
        }
    }

    /// Semantic action for non-terminal 'WithParameter'
    fn with_parameter(&mut self, arg: &WithParameter) {
        self.hash(&arg.hash);
        self.l_paren(&arg.l_paren);
        if let Some(ref x) = arg.with_parameter_opt {
            self.with_parameter_list(&x.with_parameter_list);
        }
        self.r_paren(&arg.r_paren);
    }

    /// Semantic action for non-terminal 'WithParameterList'
    fn with_parameter_list(&mut self, arg: &WithParameterList) {
        self.with_parameter_item(&arg.with_parameter_item);
        for x in &arg.with_parameter_list_list {
            self.comma(&x.comma);
            self.with_parameter_item(&x.with_parameter_item);
        }
        if let Some(ref x) = arg.with_parameter_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'WithParameterItem'
    fn with_parameter_item(&mut self, arg: &WithParameterItem) {
        match &*arg.with_parameter_item_group {
            WithParameterItemGroup::Parameter(x) => self.parameter(&x.parameter),
            WithParameterItemGroup::Localparam(x) => self.localparam(&x.localparam),
        };
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.r#type(&arg.r#type);
        self.equ(&arg.equ);
        self.expression(&arg.expression);
    }

    /// Semantic action for non-terminal 'PortDeclaration'
    fn port_declaration(&mut self, arg: &PortDeclaration) {
        self.l_paren(&arg.l_paren);
        if let Some(ref x) = arg.port_declaration_opt {
            self.port_declaration_list(&x.port_declaration_list);
        }
        self.r_paren(&arg.r_paren);
    }

    /// Semantic action for non-terminal 'PortDeclarationList'
    fn port_declaration_list(&mut self, arg: &PortDeclarationList) {
        self.port_declaration_item(&arg.port_declaration_item);
        for x in &arg.port_declaration_list_list {
            self.comma(&x.comma);
            self.port_declaration_item(&x.port_declaration_item);
        }
        if let Some(ref x) = arg.port_declaration_list_opt {
            self.comma(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'PortDeclarationItem'
    fn port_declaration_item(&mut self, arg: &PortDeclarationItem) {
        self.identifier(&arg.identifier);
        self.colon(&arg.colon);
        self.direction(&arg.direction);
        self.r#type(&arg.r#type);
    }

    /// Semantic action for non-terminal 'Direction'
    fn direction(&mut self, arg: &Direction) {
        match arg {
            Direction::Input(x) => self.input(&x.input),
            Direction::Output(x) => self.output(&x.output),
            Direction::Inout(x) => self.inout(&x.inout),
            Direction::Ref(x) => self.r#ref(&x.r#ref),
        };
    }

    /// Semantic action for non-terminal 'FunctionDeclaration'
    fn function_declaration(&mut self, arg: &FunctionDeclaration) {
        self.function(&arg.function);
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.function_declaration_opt {
            self.with_parameter(&x.with_parameter);
        }
        if let Some(ref x) = arg.function_declaration_opt0 {
            self.port_declaration(&x.port_declaration);
        }
        self.minus_g_t(&arg.minus_g_t);
        self.r#type(&arg.r#type);
        self.l_brace(&arg.l_brace);
        for x in &arg.function_declaration_list {
            self.function_item(&x.function_item);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'FunctionItem'
    fn function_item(&mut self, arg: &FunctionItem) {
        match arg {
            FunctionItem::LetDeclaration(x) => self.let_declaration(&x.let_declaration),
            FunctionItem::Statement(x) => self.statement(&x.statement),
        };
    }

    /// Semantic action for non-terminal 'ModuleDeclaration'
    fn module_declaration(&mut self, arg: &ModuleDeclaration) {
        self.module(&arg.module);
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.module_declaration_opt {
            self.with_parameter(&x.with_parameter);
        }
        if let Some(ref x) = arg.module_declaration_opt0 {
            self.port_declaration(&x.port_declaration);
        }
        self.l_brace(&arg.l_brace);
        for x in &arg.module_declaration_list {
            self.module_item(&x.module_item);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'ModuleIfDeclaration'
    fn module_if_declaration(&mut self, arg: &ModuleIfDeclaration) {
        self.r#if(&arg.r#if);
        self.expression(&arg.expression);
        self.colon(&arg.colon);
        self.identifier(&arg.identifier);
        self.l_brace(&arg.l_brace);
        for x in &arg.module_if_declaration_list {
            self.module_item(&x.module_item);
        }
        self.r_brace(&arg.r_brace);
        for x in &arg.module_if_declaration_list0 {
            self.r#else(&x.r#else);
            self.r#if(&x.r#if);
            self.expression(&x.expression);
            if let Some(ref x) = x.module_if_declaration_opt {
                self.colon(&x.colon);
                self.identifier(&x.identifier);
            }
            self.l_brace(&x.l_brace);
            for x in &x.module_if_declaration_list0_list {
                self.module_item(&x.module_item);
            }
            self.r_brace(&x.r_brace);
        }
        if let Some(ref x) = arg.module_if_declaration_opt0 {
            self.r#else(&x.r#else);
            if let Some(ref x) = x.module_if_declaration_opt1 {
                self.colon(&x.colon);
                self.identifier(&x.identifier);
            }
            self.l_brace(&x.l_brace);
            for x in &x.module_if_declaration_opt0_list {
                self.module_item(&x.module_item);
            }
            self.r_brace(&x.r_brace);
        }
    }

    /// Semantic action for non-terminal 'ModuleForDeclaration'
    fn module_for_declaration(&mut self, arg: &ModuleForDeclaration) {
        self.r#for(&arg.r#for);
        self.identifier(&arg.identifier);
        self.r#in(&arg.r#in);
        self.expression(&arg.expression);
        self.dot_dot(&arg.dot_dot);
        self.expression(&arg.expression0);
        if let Some(ref x) = arg.module_for_declaration_opt {
            self.step(&x.step);
            self.assignment_operator(&x.assignment_operator);
            self.expression(&x.expression);
        }
        self.colon(&arg.colon);
        self.identifier(&arg.identifier0);
        self.l_brace(&arg.l_brace);
        for x in &arg.module_for_declaration_list {
            self.module_item(&x.module_item);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'ModuleItem'
    fn module_item(&mut self, arg: &ModuleItem) {
        match arg {
            ModuleItem::LetDeclaration(x) => self.let_declaration(&x.let_declaration),
            ModuleItem::ParameterDeclaration(x) => {
                self.parameter_declaration(&x.parameter_declaration)
            }
            ModuleItem::LocalparamDeclaration(x) => {
                self.localparam_declaration(&x.localparam_declaration)
            }
            ModuleItem::AlwaysFfDeclaration(x) => {
                self.always_ff_declaration(&x.always_ff_declaration)
            }
            ModuleItem::AlwaysCombDeclaration(x) => {
                self.always_comb_declaration(&x.always_comb_declaration)
            }
            ModuleItem::AssignDeclaration(x) => self.assign_declaration(&x.assign_declaration),
            ModuleItem::FunctionDeclaration(x) => {
                self.function_declaration(&x.function_declaration)
            }
            ModuleItem::ModuleIfDeclaration(x) => {
                self.module_if_declaration(&x.module_if_declaration)
            }
            ModuleItem::ModuleForDeclaration(x) => {
                self.module_for_declaration(&x.module_for_declaration)
            }
            ModuleItem::EnumDeclaration(x) => self.enum_declaration(&x.enum_declaration),
            ModuleItem::StructDeclaration(x) => self.struct_declaration(&x.struct_declaration),
        };
    }

    /// Semantic action for non-terminal 'InterfaceDeclaration'
    fn interface_declaration(&mut self, arg: &InterfaceDeclaration) {
        self.interface(&arg.interface);
        self.identifier(&arg.identifier);
        if let Some(ref x) = arg.interface_declaration_opt {
            self.with_parameter(&x.with_parameter);
        }
        self.l_brace(&arg.l_brace);
        for x in &arg.interface_declaration_list {
            self.interface_item(&x.interface_item);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'InterfaceIfDeclaration'
    fn interface_if_declaration(&mut self, arg: &InterfaceIfDeclaration) {
        self.r#if(&arg.r#if);
        self.expression(&arg.expression);
        self.colon(&arg.colon);
        self.identifier(&arg.identifier);
        self.l_brace(&arg.l_brace);
        for x in &arg.interface_if_declaration_list {
            self.interface_item(&x.interface_item);
        }
        self.r_brace(&arg.r_brace);
        for x in &arg.interface_if_declaration_list0 {
            self.r#else(&x.r#else);
            self.r#if(&x.r#if);
            self.expression(&x.expression);
            if let Some(ref x) = x.interface_if_declaration_opt {
                self.colon(&x.colon);
                self.identifier(&x.identifier);
            }
            self.l_brace(&x.l_brace);
            for x in &x.interface_if_declaration_list0_list {
                self.interface_item(&x.interface_item);
            }
            self.r_brace(&x.r_brace);
        }
        if let Some(ref x) = arg.interface_if_declaration_opt0 {
            self.r#else(&x.r#else);
            if let Some(ref x) = x.interface_if_declaration_opt1 {
                self.colon(&x.colon);
                self.identifier(&x.identifier);
            }
            self.l_brace(&x.l_brace);
            for x in &x.interface_if_declaration_opt0_list {
                self.interface_item(&x.interface_item);
            }
            self.r_brace(&x.r_brace);
        }
    }

    /// Semantic action for non-terminal 'InterfaceForDeclaration'
    fn interface_for_declaration(&mut self, arg: &InterfaceForDeclaration) {
        self.r#for(&arg.r#for);
        self.identifier(&arg.identifier);
        self.r#in(&arg.r#in);
        self.expression(&arg.expression);
        self.dot_dot(&arg.dot_dot);
        self.expression(&arg.expression0);
        if let Some(ref x) = arg.interface_for_declaration_opt {
            self.step(&x.step);
            self.assignment_operator(&x.assignment_operator);
            self.expression(&x.expression);
        }
        self.colon(&arg.colon);
        self.identifier(&arg.identifier0);
        self.l_brace(&arg.l_brace);
        for x in &arg.interface_for_declaration_list {
            self.interface_item(&x.interface_item);
        }
        self.r_brace(&arg.r_brace);
    }

    /// Semantic action for non-terminal 'InterfaceItem'
    fn interface_item(&mut self, arg: &InterfaceItem) {
        match arg {
            InterfaceItem::LetDeclaration(x) => self.let_declaration(&x.let_declaration),
            InterfaceItem::ParameterDeclaration(x) => {
                self.parameter_declaration(&x.parameter_declaration)
            }
            InterfaceItem::LocalparamDeclaration(x) => {
                self.localparam_declaration(&x.localparam_declaration)
            }
            InterfaceItem::ModportDeclaration(x) => {
                self.modport_declaration(&x.modport_declaration)
            }
            InterfaceItem::InterfaceIfDeclaration(x) => {
                self.interface_if_declaration(&x.interface_if_declaration)
            }
            InterfaceItem::InterfaceForDeclaration(x) => {
                self.interface_for_declaration(&x.interface_for_declaration)
            }
            InterfaceItem::EnumDeclaration(x) => self.enum_declaration(&x.enum_declaration),
            InterfaceItem::StructDeclaration(x) => self.struct_declaration(&x.struct_declaration),
        };
    }

    /// Semantic action for non-terminal 'Description'
    fn description(&mut self, arg: &Description) {
        match arg {
            Description::ModuleDeclaration(x) => self.module_declaration(&x.module_declaration),
            Description::InterfaceDeclaration(x) => {
                self.interface_declaration(&x.interface_declaration)
            }
        };
    }

    /// Semantic action for non-terminal 'Veryl'
    fn veryl(&mut self, arg: &Veryl) {
        self.start(&arg.start);
        for x in &arg.veryl_list {
            self.description(&x.description);
        }
    }
}
