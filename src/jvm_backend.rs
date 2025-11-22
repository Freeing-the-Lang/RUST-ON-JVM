use crate::ir::IRFunction;
use crate::classfile::ClassFile;

pub fn emit_classfile(ir: &IRFunction, path: &str) {
    let bytecode = generate_bytecode(ir);
    let cf = ClassFile::new(ir, bytecode);
    cf.write(path);
}

pub fn generate_bytecode(ir: &IRFunction) -> Vec<u8> {
    let mut code = Vec::new();

    for instr in &ir.instr {
        match instr.as_str() {
            "LOAD_0" => code.push(0x1a),
            "LOAD_1" => code.push(0x1b),
            "ADD"    => code.push(0x60),
            "RETURN" => code.push(0xac),
            _ => {}
        }
    }

    code
}
