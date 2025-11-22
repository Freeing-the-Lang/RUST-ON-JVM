#[derive(Debug)]
pub enum CpEntry {
    Utf8(String),
    Class(u16),
    NameAndType { name_index: u16, type_index: u16 },
    MethodRef { class_index: u16, nat_index: u16 },
}

#[derive(Debug)]
pub struct ConstantPool {
    pub entries: Vec<CpEntry>,
}

impl ConstantPool {
    pub fn new() -> Self {
        // index 0은 비움(더미)
        Self {
            entries: vec![CpEntry::Utf8("dummy".into())],
        }
    }

    // ===== UTF-8 문자열 추가 =====
    pub fn add_utf8(&mut self, s: &str) -> u16 {
        self.entries.push(CpEntry::Utf8(s.into()));
        (self.entries.len() - 1) as u16
    }

    // ===== Class 정보 추가 =====
    pub fn add_class(&mut self, name_index: u16) -> u16 {
        self.entries.push(CpEntry::Class(name_index));
        (self.entries.len() - 1) as u16
    }

    // ===== NameAndType 추가 =====
    pub fn add_name_and_type(&mut self, name_index: u16, type_index: u16) -> u16 {
        self.entries.push(CpEntry::NameAndType {
            name_index,
            type_index,
        });
        (self.entries.len() - 1) as u16
    }

    // ===== MethodRef 추가 =====
    pub fn add_methodref(&mut self, class_index: u16, nat_index: u16) -> u16 {
        self.entries.push(CpEntry::MethodRef {
            class_index,
            nat_index,
        });
        (self.entries.len() - 1) as u16
    }

    // ===== Constant Pool 전체를 JVM ClassFile 바이트로 변환 =====
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // constant_pool_count = entry_count + 1
        let count = (self.entries.len() + 1) as u16;
        buf.extend_from_slice(&count.to_be_bytes());

        for entry in &self.entries {
            match entry {
                CpEntry::Utf8(s) => {
                    buf.push(1); // UTF8 tag
                    buf.extend_from_slice(&(s.len() as u16).to_be_bytes());
                    buf.extend_from_slice(s.as_bytes());
                }

                CpEntry::Class(idx) => {
                    buf.push(7); // Class tag
                    buf.extend_from_slice(&idx.to_be_bytes());
                }

                CpEntry::NameAndType { name_index, type_index } => {
                    buf.push(12); // NameAndType tag
                    buf.extend_from_slice(&name_index.to_be_bytes());
                    buf.extend_from_slice(&type_index.to_be_bytes());
                }

                CpEntry::MethodRef { class_index, nat_index } => {
                    buf.push(10); // MethodRef tag
                    buf.extend_from_slice(&class_index.to_be_bytes());
                    buf.extend_from_slice(&nat_index.to_be_bytes());
                }
            }
        }

        buf
    }
}
