use std::fs;
use crate::parser::parse_operator_file;
use crate::ast::OperatorDef;

pub fn load_system_operators() -> Vec<OperatorDef> {
    let code = fs::read_to_string("flux/ops/operator.fx").expect("Failed to read operator.fx");
    parse_operator_file(&code)
}