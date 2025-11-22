use crate::parser::AstFunction;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<String>,
    pub ret_type: String,
    pub instr: Vec<String>,
}

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
