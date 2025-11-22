#[derive(Debug)]
pub enum CpEntry {
    Utf8(String),
    Class(u16),
    NameAndType { name_index: u16, type_index: u16 },
    MethodRef { class_index: u16, nat_index: u16 },
}

impl ConstantPool {
    pub fn add_methodref(&mut self, class_idx: u16, nat_idx: u16) -> u16 {
        self.entries.push(CpEntry::MethodRef {
            class_index: class_idx,
            nat_index: nat_idx,
        });
        (self.entries.len() - 1) as u16
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        let count = (self.entries.len() + 1) as u16;
        buf.extend_from_slice(&count.to_be_bytes());

        for entry in &self.entries {
            match entry {
                CpEntry::Utf8(s) => {
                    buf.push(1);
                    buf.extend_from_slice(&(s.len() as u16).to_be_bytes());
                    buf.extend_from_slice(s.as_bytes());
                }
                CpEntry::Class(idx) => {
                    buf.push(7);
                    buf.extend_from_slice(&idx.to_be_bytes());
                }
                CpEntry::NameAndType { name_index, type_index } => {
                    buf.push(12);
                    buf.extend_from_slice(&name_index.to_be_bytes());
                    buf.extend_from_slice(&type_index.to_be_bytes());
                }
                CpEntry::MethodRef { class_index, nat_index } => {
                    buf.push(10); // Methodref tag
                    buf.extend_from_slice(&class_index.to_be_bytes());
                    buf.extend_from_slice(&nat_index.to_be_bytes());
                }
            }
        }

        buf
    }
}
