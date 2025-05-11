use crate::{expression::Expression, Token};
use alloc::vec::Vec;

fn is_at_end(length: usize, current: usize) -> bool {
    current >= length
}

//fn term(tokens: &Vec<Token>, current: &usize) -> &impl Expression {}

pub fn parse(tokens: Vec<Token>) {
    let length: usize = tokens.len();
    let mut current: usize = 0;

    //term(&tokens, &current);
}
