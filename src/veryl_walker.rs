use crate::veryl_grammar_trait::*;
use crate::veryl_token::VerylToken;

pub trait VerylWalker {
    fn empty(&mut self, _: &Empty) {}

    fn operator(&mut self, arg: &Operator) {
        self.veryl_token(&arg.operator_token);
    }

    fn empty_operator(&mut self, _: &Operator) {}

    /// Semantic action for non-terminal 'VerylToken'
    fn veryl_token(&mut self, _arg: &VerylToken) {}

    /// Semantic action for non-terminal 'HierarchicalIdentifier'
    fn hierarchical_identifier(&mut self, arg: &HierarchicalIdentifier) {
        self.empty(&arg.identifier);
        for x in &arg.hierarchical_identifier_list {
            self.range(&x.range);
        }
        for x in &arg.hierarchical_identifier_list0 {
            self.empty(&x.dot);
            self.empty(&x.identifier);
            for x in &x.hierarchical_identifier_list0_list {
                self.range(&x.range);
            }
        }
    }

    /// Semantic action for non-terminal 'Expression'
    fn expression(&mut self, arg: &Expression) {
        self.expression01(&arg.expression01);
        for x in &arg.expression_list {
            self.operator(&x.operator01);
            self.expression01(&x.expression01);
        }
    }

    /// Semantic action for non-terminal 'Expression01'
    fn expression01(&mut self, arg: &Expression01) {
        self.expression02(&arg.expression02);
        for x in &arg.expression01_list {
            self.operator(&x.operator02);
            self.expression02(&x.expression02);
        }
    }

    /// Semantic action for non-terminal 'Expression02'
    fn expression02(&mut self, arg: &Expression02) {
        self.expression03(&arg.expression03);
        for x in &arg.expression02_list {
            self.operator(&x.operator03);
            self.expression03(&x.expression03);
        }
    }

    /// Semantic action for non-terminal 'Expression03'
    fn expression03(&mut self, arg: &Expression03) {
        self.expression04(&arg.expression04);
        for x in &arg.expression03_list {
            self.operator(&x.operator04);
            self.expression04(&x.expression04);
        }
    }

    /// Semantic action for non-terminal 'Expression04'
    fn expression04(&mut self, arg: &Expression04) {
        self.expression05(&arg.expression05);
        for x in &arg.expression04_list {
            self.operator(&x.operator05);
            self.expression05(&x.expression05);
        }
    }

    /// Semantic action for non-terminal 'Expression05'
    fn expression05(&mut self, arg: &Expression05) {
        self.expression06(&arg.expression06);
        for x in &arg.expression05_list {
            self.operator(&x.operator06);
            self.expression06(&x.expression06);
        }
    }

    /// Semantic action for non-terminal 'Expression06'
    fn expression06(&mut self, arg: &Expression06) {
        self.expression07(&arg.expression07);
        for x in &arg.expression06_list {
            self.operator(&x.operator07);
            self.expression07(&x.expression07);
        }
    }

    /// Semantic action for non-terminal 'Expression07'
    fn expression07(&mut self, arg: &Expression07) {
        self.expression08(&arg.expression08);
        for x in &arg.expression07_list {
            self.operator(&x.operator08);
            self.expression08(&x.expression08);
        }
    }

    /// Semantic action for non-terminal 'Expression08'
    fn expression08(&mut self, arg: &Expression08) {
        self.expression09(&arg.expression09);
        for x in &arg.expression08_list {
            self.operator(&x.operator09);
            self.expression09(&x.expression09);
        }
    }

    /// Semantic action for non-terminal 'Expression09'
    fn expression09(&mut self, arg: &Expression09) {
        self.expression10(&arg.expression10);
        for x in &arg.expression09_list {
            self.operator(&x.operator10);
            self.expression10(&x.expression10);
        }
    }

    /// Semantic action for non-terminal 'Expression10'
    fn expression10(&mut self, arg: &Expression10) {
        self.expression11(&arg.expression11);
        for x in &arg.expression10_list {
            self.operator(&x.operator11);
            self.expression11(&x.expression11);
        }
    }

    /// Semantic action for non-terminal 'Expression11'
    fn expression11(&mut self, arg: &Expression11) {
        if let Some(ref x) = arg.expression11_opt {
            match &*x.expression11_opt_group {
                Expression11OptGroup::Operator03(x) => self.operator(&x.operator),
                Expression11OptGroup::Operator04(x) => self.operator(&x.operator),
                Expression11OptGroup::Operator05(x) => self.operator(&x.operator),
                Expression11OptGroup::Operator09(x) => self.operator(&x.operator),
            }
        }
        self.factor(&arg.factor);
    }

    /// Semantic action for non-terminal 'Factor'
    fn factor(&mut self, arg: &Factor) {
        match arg {
            Factor::Number(x) => self.empty(&x.number),
            Factor::FactorOptHierarchicalIdentifierFactorOpt0(x) => {
                if let Some(ref x) = x.factor_opt {
                    self.empty(&x.dollar);
                }
                self.hierarchical_identifier(&x.hierarchical_identifier);
                if let Some(ref x) = x.factor_opt0 {
                    self.empty(&x.l_paren);
                    if let Some(ref x) = x.factor_opt1 {
                        self.function_call_arg(&x.function_call_arg);
                    }
                    self.empty(&x.r_paren);
                }
            }
            Factor::LParenExpressionRParen(x) => {
                self.empty(&x.l_paren);
                self.expression(&x.expression);
                self.empty(&x.r_paren);
            }
        }
    }

    /// Semantic action for non-terminal 'FunctionCallArg'
    fn function_call_arg(&mut self, arg: &FunctionCallArg) {
        self.expression(&arg.expression);
        for x in &arg.function_call_arg_list {
            self.empty(&x.comma);
            self.expression(&x.expression);
        }
        if let Some(ref x) = arg.function_call_arg_opt {
            self.empty(&x.comma);
        }
    }

    /// Semantic action for non-terminal 'Range'
    fn range(&mut self, arg: &Range) {
        self.empty(&arg.l_bracket);
        self.expression(&arg.expression);
        if let Some(ref x) = arg.range_opt {
            self.empty(&x.range_operator);
            self.expression(&x.expression);
        }
        self.empty(&arg.r_bracket);
    }
}
