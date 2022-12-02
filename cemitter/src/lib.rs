use std::{fs::File, io::Write};

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Self {
        Emitter {
            full_path,
            header: "".to_string(),
            code: "".to_string(),
        }
    }

    pub fn emit(&mut self, code: String) {
        self.code.push_str(&code);
    }

    pub fn emit_line(&mut self, code: String) {
        self.code.push_str(&code);
        self.code.push('\n');
    }

    pub fn header_line(&mut self, code: String) {
        self.header.push_str(&code);
        self.header.push('\n');
    }

    pub fn write_file(&mut self) {
        // 'borrowed' from sgmarz's implementation
        if let Ok(mut f) = File::create(&self.full_path) {
            // unwraps are kind of okay because error checking on outside
            f.write_all(self.header.as_bytes()).unwrap();
            f.write_all(self.code.as_bytes()).unwrap();
            f.sync_all().unwrap();
        } else {
            panic!("Could not open file for writing: '{}'", self.full_path);
        }
    }
}
