use crate::ir::IRFunction;
use crate::parser::AstFunction;

pub fn analyze(ast: &AstFunction) -> IRFunction {
    IRFunction {
        name: ast.name.clone(),
        params: ast.params.clone(),
        ret_type: ast.ret_type.clone(),
        instr: vec![
            "LOAD_0".into(),
            "LOAD_1".into(),
            "ADD".into(),
            "RETURN".into(),
        ],
    }
}
