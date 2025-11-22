use crate::constant_pool::ConstantPool;
use crate::ir::IRFunction;
use crate::type_descriptor::rust_fn_sig_to_jvm;

use std::fs::File;
use std::io::Write;

pub struct ClassFile {
    pub cp: ConstantPool,
    pub bytecode: Vec<u8>,
}

impl ClassFile {
    pub fn new(ir: &IRFunction, bytecode: Vec<u8>) -> Self {
        let mut cp = ConstantPool::new();

        let class_name = cp.add_utf8("Add");
        let class_ref = cp.add_class(class_name);

        let name_idx = cp.add_utf8(&ir.name);
        let desc_str = rust_fn_sig_to_jvm(&ir.params, &ir.ret_type);
        let desc_idx = cp.add_utf8(&desc_str);

        let nat_idx = cp.add_name_and_type(name_idx, desc_idx);

        cp.add_methodref(class_ref, nat_idx);

        Self { cp, bytecode }
    }

    pub fn write(&self, path: &str) {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x34]);

        bytes.extend_from_slice(&self.cp.to_bytes());

        bytes.extend_from_slice(&[0x00, 0x21]);
        bytes.extend_from_slice(&[0x00, 0x01, 0x00, 0x01]);

        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00]);

        bytes.extend_from_slice(&[0x00, 0x01]);

        bytes.extend_from_slice(&[0x00, 0x09]);
        bytes.extend_from_slice(&[0x00, 0x01]);
        bytes.extend_from_slice(&[0x00, 0x01]);
        bytes.extend_from_slice(&[0x00, 0x01]);

        bytes.extend_from_slice(&[0x00, 0x01]);
        let code_len = self.bytecode.len() as u32;
        let attr_len = code_len + 12;

        bytes.extend_from_slice(&attr_len.to_be_bytes());
        bytes.extend_from_slice(&[0x00, 0x04, 0x00, 0x04]);
        bytes.extend_from_slice(&code_len.to_be_bytes());
        bytes.extend_from_slice(&self.bytecode);

        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00]);

        bytes.extend_from_slice(&[0x00, 0x00]);

        let mut f = File::create(path).unwrap();
        f.write_all(&bytes).unwrap();
    }
}
