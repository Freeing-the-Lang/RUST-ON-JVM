#[derive(Debug)]
pub enum CpEntry {
    Utf8(String),
    Class(u16),
    NameAndType { name_index: u16, type_index: u16 },
}

#[derive(Debug)]
pub struct ConstantPool {
    pub entries: Vec<CpEntry>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self { entries: vec![CpEntry::Utf8("dummy".into())] }
        // index 0은 사용하지 않기 때문에 dummy 넣음
    }

    pub fn add_utf8(&mut self, s: &str) -> u16 {
        self.entries.push(CpEntry::Utf8(s.into()));
        (self.entries.len() - 1) as u16
    }

    pub fn add_class(&mut self, name_index: u16) -> u16 {
        self.entries.push(CpEntry::Class(name_index));
        (self.entries.len() - 1) as u16
    }

    pub fn add_name_and_type(&mut self, name_index: u16, type_index: u16) -> u16 {
        self.entries.push(CpEntry::NameAndType {
            name_index,
            type_index,
        });
        (self.entries.len() - 1) as u16
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // constant_pool_count = entry count + 1
        let count = (self.entries.len() + 1) as u16;
        buf.extend_from_slice(&count.to_be_bytes());

        for entry in &self.entries {
            match entry {
                CpEntry::Utf8(s) => {
                    buf.push(1); // tag
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
            }
        }

        buf
    }
}
