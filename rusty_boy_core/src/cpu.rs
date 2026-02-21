mod opcodes;

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL,
}

#[derive(Clone, Copy)]
enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

struct Cpu {
    sp: u16,
    pc: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

enum Flags {
    Z,
    N,
    H,
    C,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            sp: 0x0000,
            pc: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.read_ram(self.pc);
        self.pc += 1;
        val
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();

        merge_bytes(high, low)
    }

    fn write_ram(&mut self, pc: u16, value: u8) {
        todo!()
    }

    fn read_ram(&self, pc: u16) -> u8 {
        todo!()
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Z => self.f & 0b1000_0000 != 0,
            Flags::N => self.f & 0b0100_0000 != 0,
            Flags::H => self.f & 0b0010_0000 != 0,
            Flags::C => self.f & 0b0001_0000 != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            match flag {
                Flags::Z => self.f |= 0b1000_0000,
                Flags::N => self.f |= 0b0100_0000,
                Flags::H => self.f |= 0b0010_0000,
                Flags::C => self.f |= 0b0001_0000,
            }
        } else {
            match flag {
                Flags::Z => self.f &= 0b0111_0000,
                Flags::N => self.f &= 0b1011_0000,
                Flags::H => self.f &= 0b1101_0000,
                Flags::C => self.f &= 0b1110_0000,
            }
        }
    }

    pub fn get_u16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => merge_bytes(self.a, self.f),
            Register16::BC => merge_bytes(self.b, self.c),
            Register16::DE => merge_bytes(self.d, self.e),
            Register16::HL => merge_bytes(self.h, self.l),
            Register16::SP => self.sp,
        }
    }

    pub fn set_u16(&mut self, reg: Register16, value: u16) {
        let high = get_high_byte(value);
        let low = get_low_byte(value);

        match reg {
            Register16::AF => {
                self.set_u8(Register::A, high);
                self.set_u8(Register::F, low);
            }
            Register16::BC => {
                self.set_u8(Register::B, high);
                self.set_u8(Register::C, low);
            }
            Register16::DE => {
                self.set_u8(Register::D, high);
                self.set_u8(Register::E, low);
            }
            Register16::HL => {
                self.set_u8(Register::H, high);
                self.set_u8(Register::L, low);
            }
            Register16::SP => {
                self.sp = value;
            }
        };
    }

    pub fn get_u8(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
            Register::HL => {
                let addr = self.get_u16(Register16::HL);
                self.read_ram(addr)
            }
        }
    }

    pub fn set_u8(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f = value & 0xF0, // Bitwise to keep the upper 4 bits of flags and discard the lower
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::HL => {
                let addr = self.get_u16(Register16::HL);
                self.write_ram(addr, value)
            }
        };
    }

    pub fn dec_u8(&mut self, r: Register) {
        let val = self.get_u8(r);
        let dec = val.wrapping_sub(1);
        let set_h = check_half_borrow_u8(val, 1);

        self.set_u8(r, dec);
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::Z, dec == 0);
        self.set_flag(Flags::H, set_h);
    }

    pub fn inc_u8(&mut self, r: Register) {
        let val = self.get_u8(r);
        let inc = val.wrapping_add(1);
        let set_h = check_half_carry_u8(val, 1);

        self.set_u8(r, inc);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::Z, inc == 0);
        self.set_flag(Flags::H, set_h);
    }

    pub fn dec_r16(&mut self, reg: Register16) {
        let val = self.get_u16(reg);
        let dec = val.wrapping_sub(1);
        self.set_u16(reg, dec);
    }

    pub fn inc_r16(&mut self, reg: Register16) {
        let val = self.get_u16(reg);
        let dec = val.wrapping_add(1);
        self.set_u16(reg, dec);
    }
}

fn merge_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | low as u16
}

fn get_low_byte(value: u16) -> u8 {
    (value & 0xFF) as u8
}

fn get_high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}

fn check_carry_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_add(rhs).is_none()
}

fn check_carry_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_add(rhs).is_none()
}

fn check_borrow_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_sub(rhs).is_none()
}

fn check_borrow_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_sub(rhs).is_none()
}

fn check_half_carry_u8(lhs: u8, rhs: u8) -> bool {
    ((lhs & 0xF) + (rhs & 0xF)) & 0xF0 != 0
}

fn check_half_carry_u16(lhs: u16, rhs: u16) -> bool {
    ((lhs & 0xFFF) + (rhs & 0xFFF)) & 0xF000 != 0
}

fn check_half_borrow_u8(lhs: u8, rhs: u8) -> bool {
    (lhs & 0xF).checked_sub(rhs & 0xF).is_none()
}

fn check_half_borrow_u16(lhs: u16, rhs: u16) -> bool {
    (lhs & 0xFFF).checked_sub(rhs & 0xFFF).is_none()
}
