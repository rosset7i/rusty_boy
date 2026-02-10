struct MemoryBus {
    rom_bank_zero: [u8; 0x4000], // 16384 Bytes 16 KiB
    rom_bank_one: [u8; 0x4000],  // 16384 Bytes 16 KiB
    v_ram: [u8; 0x2000],         // 8192 Bytes 8 KiB
    external_ram: [u8; 0x2000],  // 8192 Bytes 8 KiB
    w_ram: [u8; 0x2000],         // 8192 Bytes 8 KiB
    io: [u8; 0x7f],
    h_ram: [u8; 0x7f],
}

impl MemoryBus {
    fn new() -> Self {
        Self {
            rom_bank_zero: [0; 0x4000],
            rom_bank_one: [0; 0x4000],
            v_ram: [0; 0x2000],
            external_ram: [0; 0x2000],
            w_ram: [0; 0x2000],
            io: [0; 0x7f],
            h_ram: [0; 0x7f],
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0..=0x3FFF => self.rom_bank_zero[address as usize],
            0x4000..=0x7FFF => self.rom_bank_one[(address - 0x4000) as usize],
            0x8000..=0x9FFF => self.v_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.w_ram[(address - 0xC000) as usize],
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.h_ram[(address - 0xFF80) as usize],
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0..=0x3FFF => self.rom_bank_zero[address as usize] = value,
            0x4000..=0x7FFF => self.rom_bank_one[(address - 0x4000) as usize] = value,
            0x8000..=0x9FFF => self.v_ram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.w_ram[(address - 0xC000) as usize] = value,
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.h_ram[(address - 0xFF80) as usize] = value,
            _ => {}
        }
    }

    fn load_rom(&mut self, rom_bytes: Vec<u8>) {
        for (addr, byte) in (0..).zip(rom_bytes) {
            self.write_byte(addr, byte);
        }
    }
}
