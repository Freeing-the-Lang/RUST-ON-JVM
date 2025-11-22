#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: String,
    pub params: Vec<String>,
    pub ret_type: String,
    pub body: String,
}

pub fn parse(_src: &str) -> AstFunction {
    // 실제 Rust 파싱 대신 의미 기반 더미
    AstFunction {
        name: "add".into(),
        params: vec!["i32".into(), "i32".into()],
        ret_type: "i32".into(),
        body: "a + b".into(),
    }
}
