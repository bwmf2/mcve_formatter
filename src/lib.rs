mod veryl_grammar_trait;
mod veryl_token;
mod veryl_walker;

use crate::veryl_grammar_trait::*;
use crate::veryl_token::{Token, VerylToken};
use crate::veryl_walker::VerylWalker;

#[derive(Default)]
pub struct Location {
    pub length: usize,
}

impl From<Token> for Location {
    fn from(x: Token) -> Self {
        Self { length: x.length }
    }
}

pub struct Align {
    width: usize,
    last_location: Option<Location>,
}

impl Align {
    fn token(&mut self, x: &VerylToken) {
        self.width += x.token.length;
        let loc: Location = x.token.into();
        self.last_location = Some(loc);
    }
}

pub struct Aligner {
    aligns: [Align; 2],
}

impl Aligner {
    fn space(&mut self, _: usize) {}
}

impl VerylWalker for Aligner {
    fn veryl_token(&mut self, arg: &VerylToken) {
        for i in 0..self.aligns.len() {
            self.aligns[i].token(arg);
        }
    }

    fn expression(&mut self, arg: &Expression) {
        self.expression01(&arg.expression01);
        for x in &arg.expression_list {
            self.space(1);
            self.operator(&x.operator01);
            self.space(1);
            self.expression01(&x.expression01);
        }
    }

    fn expression01(&mut self, arg: &Expression01) {
        self.expression02(&arg.expression02);
        for x in &arg.expression01_list {
            self.space(1);
            self.operator(&x.operator02);
            self.space(1);
            self.expression02(&x.expression02);
        }
    }

    fn expression02(&mut self, arg: &Expression02) {
        self.expression03(&arg.expression03);
        for x in &arg.expression02_list {
            self.space(1);
            self.operator(&x.operator03);
            self.space(1);
            self.expression03(&x.expression03);
        }
    }

    fn expression03(&mut self, arg: &Expression03) {
        self.expression04(&arg.expression04);
        for x in &arg.expression03_list {
            self.space(1);
            self.operator(&x.operator04);
            self.space(1);
            self.expression04(&x.expression04);
        }
    }

    fn expression04(&mut self, arg: &Expression04) {
        self.expression05(&arg.expression05);
        for x in &arg.expression04_list {
            self.space(1);
            self.empty_operator(&x.operator05);
            self.space(1);
            self.expression05(&x.expression05);
        }
    }

    fn expression05(&mut self, arg: &Expression05) {
        self.expression06(&arg.expression06);
        for x in &arg.expression05_list {
            self.space(1);
            self.empty_operator(&x.operator06);
            self.space(1);
            self.expression06(&x.expression06);
        }
    }

    fn expression06(&mut self, arg: &Expression06) {
        self.expression07(&arg.expression07);
        for x in &arg.expression06_list {
            self.space(1);
            self.empty_operator(&x.operator07);
            self.space(1);
            self.expression07(&x.expression07);
        }
    }

    fn expression07(&mut self, arg: &Expression07) {
        self.expression08(&arg.expression08);
        for x in &arg.expression07_list {
            self.space(1);
            self.empty_operator(&x.operator08);
            self.space(1);
            self.expression08(&x.expression08);
        }
    }

    fn expression08(&mut self, arg: &Expression08) {
        self.expression09(&arg.expression09);
        for x in &arg.expression08_list {
            self.space(1);
            self.empty_operator(&x.operator09);
            self.space(1);
            self.expression09(&x.expression09);
        }
    }

    fn expression09(&mut self, arg: &Expression09) {
        self.expression10(&arg.expression10);
        for x in &arg.expression09_list {
            self.space(1);
            self.empty_operator(&x.operator10);
            self.space(1);
            self.expression10(&x.expression10);
        }
    }

    fn expression10(&mut self, arg: &Expression10) {
        self.expression11(&arg.expression11);
        for x in &arg.expression10_list {
            self.space(1);
            self.empty_operator(&x.operator11);
            self.space(1);
            self.expression11(&x.expression11);
        }
    }
}
