#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<String>,
    pub ret_type: String,
    pub instr: Vec<String>,
}
