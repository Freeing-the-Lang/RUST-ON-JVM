use std::fs::File;
use std::io::Write;

use crate::constant_pool::ConstantPool;

pub struct ClassFile {
    pub constant_pool: ConstantPool,
    pub bytecode: Vec<u8>,
}

impl ClassFile {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let mut cp = ConstantPool::new();

        // 필수 엔트리 몇 개 추가
        let class_name = cp.add_utf8("Add");
        let class_ref = cp.add_class(class_name);

        let method_name = cp.add_utf8("add");
        let descriptor = cp.add_utf8("()I");
        let name_and_type = cp.add_name_and_type(method_name, descriptor);

        Self {
            constant_pool: cp,
            bytecode,
        }
    }

    pub fn write_to_file(&self, path: &str) {
        let mut f = File::create(path)
            .expect("Failed to create class file");
        f.write_all(&self.to_classfile_format())
            .expect("Failed to write class file");
        println!("[RUST-ON-JVM] Wrote {}", path);
    }

    fn to_classfile_format(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // magic
        bytes.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);

        // minor, major
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x34]); // Java 8

        // constant pool
        bytes.extend_from_slice(&self.constant_pool.to_bytes());

        // access_flags (public)
        bytes.extend_from_slice(&[0x00, 0x21]);

        // this_class, super_class
        bytes.extend_from_slice(&[0x00, 0x01]);
        bytes.extend_from_slice(&[0x00, 0x01]);

        // interfaces, fields = 0
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00]);

        // methods_count = 1
        bytes.extend_from_slice(&[0x00, 0x01]);

        // === method info ===
        bytes.extend_from_slice(&[0x00, 0x09]); // access: public static
        bytes.extend_from_slice(&[0x00, 0x01]); // name idx
        bytes.extend_from_slice(&[0x00, 0x01]); // descriptor idx
        bytes.extend_from_slice(&[0x00, 0x01]); // attributes_count

        // === Code attribute ===
        bytes.extend_from_slice(&[0x00, 0x01]); // attribute_name_index (fake)
        let code_len = self.bytecode.len() as u32;
        let attr_len = code_len + 12;
        bytes.extend_from_slice(&attr_len.to_be_bytes());

        bytes.extend_from_slice(&[0x00, 0x04, 0x00, 0x04]); // max_stack, locals
        bytes.extend_from_slice(&(code_len).to_be_bytes());
        bytes.extend_from_slice(&self.bytecode);

        // exception table + attributes_count
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00]);

        // class attributes_count = 0
        bytes.extend_from_slice(&[0x00, 0x00]);

        bytes
    }
}
