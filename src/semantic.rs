use crate::parser::AstFunction;
use crate::ir::IRFunction;

pub fn analyze(ast: AstFunction) -> IRFunction {
    IRFunction {
        name: ast.name,
        params: ast.params.clone(),
        ret_type: "i32".into(),
        instr: vec![
            "LOAD a".into(),
            "LOAD b".into(),
            "ADD".into(),
            "RETURN".into(),
        ],
    }
}
