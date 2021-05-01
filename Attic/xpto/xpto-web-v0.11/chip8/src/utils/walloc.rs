const PAGE_SIZE: usize = 4096; // 4 Kbytes

pub struct Box {
    memory: [u8; PAGE_SIZE],
}

impl Box {
    pub fn new() -> Self {
        Self {
            memory: [0; PAGE_SIZE],
        }
    }
    pub fn drop(self) {
        let _ = self.memory;
    }
}
