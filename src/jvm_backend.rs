use crate::semantic::IRFunction;

#[derive(Debug)]
pub struct Bytecode {
    pub code: Vec<u8>,
}

pub fn generate(ir: IRFunction) -> Bytecode {
    let mut code = Vec::new();

    // JVM opcode 예시 (실제와 다르게 단순화 버전)
    for instr in ir.instr {
        match instr.as_str() {
            "LOAD a" => code.push(0x15), // iload_0
            "LOAD b" => code.push(0x16), // iload_1
            "ADD"     => code.push(0x60), // iadd
            "RETURN"  => code.push(0xac), // ireturn
            _ => {}
        }
    }

    Bytecode { code }
}
