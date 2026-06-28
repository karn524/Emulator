pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    // メモリを作成する
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    // 1byte読み込む
    pub fn read_u8(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    // 1byte書き込む
    pub fn write_u8(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }

    // 4byte読み込んで u32 にする
    pub fn read_u32(&self, address: u32) -> u32 {
        let b0 = self.read_u8(address) as u32;
        let b1 = self.read_u8(address + 1) as u32;
        let b2 = self.read_u8(address + 2) as u32;
        let b3 = self.read_u8(address + 3) as u32;

        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    // u32 を4byteに分けて書き込む
    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.write_u8(address, (value & 0xFF) as u8);
        self.write_u8(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write_u8(address + 2, ((value >> 16) & 0xFF) as u8);
        self.write_u8(address + 3, ((value >> 24) & 0xFF) as u8);
    }
}