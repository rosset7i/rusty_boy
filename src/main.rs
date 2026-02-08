use std::{error::Error, fs};

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut bus = MemoryBus::new();
    let rom_bytes = fs::read("rom.gb")?; // Read file bytes from disk
    bus.load_rom(rom_bytes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::MemoryBus;
    use std::fs;

    #[test]
    fn should_read_nintendo_image() {
        let mut bus = MemoryBus::new();
        let rom_bytes = fs::read("rom.gb").unwrap();
        bus.load_rom(rom_bytes);

        let expected_hex = "CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D 00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99 BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E";
        assert_eq!(read_range_debug(bus, 0x0104, 0x0133), expected_hex);
    }

    #[test]
    fn should_wryte_byte_at_correct_address() {
        let mut bus = MemoryBus::new();
        bus.write_byte(0x2000, 0x2F);

        assert_eq!(bus.read_byte(0x2000), 0x2F);
    }

    fn read_range_debug(bus: MemoryBus, start: u16, end: u16) -> String {
        (start..=end)
            .map(|addr| bus.read_byte(addr))
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
