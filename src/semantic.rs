use crate::parser::AstFunction;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub instr: Vec<String>, // 의미 단위 instruction
}

pub fn analyze(ast: AstFunction) -> IRFunction {
    IRFunction {
        name: ast.name,
        instr: vec![
            "LOAD a".into(),
            "LOAD b".into(),
            "ADD".into(),
            "RETURN".into(),
        ],
    }
}
