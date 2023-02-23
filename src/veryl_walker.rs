use crate::veryl_grammar_trait::*;
use crate::veryl_token::VerylToken;

pub trait VerylWalker {
    fn operator(&mut self, arg: &Operator) {
        self.veryl_token(&arg.operator_token);
    }

    fn empty_operator(&mut self, _: &Operator) {}

    fn veryl_token(&mut self, _arg: &VerylToken) {}

    fn expression(&mut self, arg: &Expression) {
        self.expression01(&arg.expression01);
        for x in &arg.expression_list {
            self.operator(&x.operator01);
            self.expression01(&x.expression01);
        }
    }

    fn expression01(&mut self, arg: &Expression01) {
        self.expression02(&arg.expression02);
        for x in &arg.expression01_list {
            self.operator(&x.operator02);
            self.expression02(&x.expression02);
        }
    }

    fn expression02(&mut self, arg: &Expression02) {
        self.expression03(&arg.expression03);
        for x in &arg.expression02_list {
            self.operator(&x.operator03);
            self.expression03(&x.expression03);
        }
    }

    fn expression03(&mut self, arg: &Expression03) {
        self.expression04(&arg.expression04);
        for x in &arg.expression03_list {
            self.operator(&x.operator04);
            self.expression04(&x.expression04);
        }
    }

    fn expression04(&mut self, arg: &Expression04) {
        self.expression05(&arg.expression05);
        for x in &arg.expression04_list {
            self.operator(&x.operator05);
            self.expression05(&x.expression05);
        }
    }

    fn expression05(&mut self, arg: &Expression05) {
        self.expression06(&arg.expression06);
        for x in &arg.expression05_list {
            self.operator(&x.operator06);
            self.expression06(&x.expression06);
        }
    }

    fn expression06(&mut self, arg: &Expression06) {
        self.expression07(&arg.expression07);
        for x in &arg.expression06_list {
            self.operator(&x.operator07);
            self.expression07(&x.expression07);
        }
    }

    fn expression07(&mut self, arg: &Expression07) {
        self.expression08(&arg.expression08);
        for x in &arg.expression07_list {
            self.operator(&x.operator08);
            self.expression08(&x.expression08);
        }
    }

    fn expression08(&mut self, arg: &Expression08) {
        self.expression09(&arg.expression09);
        for x in &arg.expression08_list {
            self.operator(&x.operator09);
            self.expression09(&x.expression09);
        }
    }

    fn expression09(&mut self, arg: &Expression09) {
        self.expression10(&arg.expression10);
        for x in &arg.expression09_list {
            self.operator(&x.operator10);
            self.expression10(&x.expression10);
        }
    }

    fn expression10(&mut self, arg: &Expression10) {
        self.expression11(&arg.expression11);
        for x in &arg.expression10_list {
            self.operator(&x.operator11);
            self.expression11(&x.expression11);
        }
    }

    fn expression11(&mut self, arg: &Expression11) {
        match &*arg.expression11_opt {
            Expression11OptGroup::Operator03(x) => self.operator(&*x),
            Expression11OptGroup::Operator04(x) => self.operator(&*x),
            Expression11OptGroup::Operator05(x) => self.operator(&*x),
            Expression11OptGroup::Operator09(x) => self.operator(&*x),
        }
        self.factor(&arg.factor);
    }

    fn factor(&mut self, arg: &Factor) {
        match arg {
            Factor::A(x) => {
                for x in x {
                    self.expression(x);
                }
            }
            Factor::B(x) => {
                for x in x {
                    self.expression(x);
                }
            }
            Factor::C(x) => {
                for x in x {
                    self.expression(x);
                }
            }
        }
        // Faster
        // match arg {
        //     Factor::A(x) => {
        //         if let Some(ref x) = *x {
        //             if let Some(ref x) = **x {
        //                 self.expression(&**x)
        //             }
        //         }
        //     }
        //     Factor::B(x) => {
        //         if let Some(ref x) = *x {
        //             if let Some(ref x) = **x {
        //                 self.expression(&**x)
        //             }
        //         }
        //     }
        //     Factor::C(x) => {
        //         if let Some(ref x) = *x {
        //             if let Some(ref x) = **x {
        //                 self.expression(&**x)
        //             }
        //         }
        //     }
        //     Factor::D(x) => {
        //         if let Some(ref x) = *x {
        //             if let Some(ref x) = **x {
        //                 self.expression(&**x)
        //             }
        //         }
        //     }
        // }
    }
}
