use crate::type_descriptor::rust_fn_sig_to_jvm;
use crate::type_descriptor::rust_type_to_jvm;

impl ClassFile {
    pub fn new(ir: &IRFunction, bytecode: Vec<u8>) -> Self {
        let mut cp = ConstantPool::new();

        let class_name = cp.add_utf8("Add");
        let _cls = cp.add_class(class_name);

        // 함수 이름 & descriptor
        let name_idx = cp.add_utf8(&ir.name);

        // descriptor 생성
        let param_refs: Vec<&str> = ir.params.iter().map(|s| s.as_str()).collect();
        let desc_str = rust_fn_sig_to_jvm(&param_refs, &ir.ret_type);

        let desc_idx = cp.add_utf8(&desc_str);

        let _nat = cp.add_name_and_type(name_idx, desc_idx);

        Self {
            constant_pool: cp,
            bytecode,
        }
    }
}
