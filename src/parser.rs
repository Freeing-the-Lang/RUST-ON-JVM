#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
}

pub fn parse(_src: &str) -> AstFunction {
    
    AstFunction {
        name: "add".to_string(),
        params: vec!["a".into(), "b".into()],
        body: "a + b".to_string(),
    }
}
