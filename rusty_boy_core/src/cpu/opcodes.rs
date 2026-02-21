use crate::cpu::{Cpu, Register, Register16};

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    todo, todo, todo, inc_03, inc_04, dec_05, todo, todo, todo, todo, todo, dec_0b, inc_0c, dec_0d,
    todo, todo, // 0x00
    todo, todo, todo, inc_13, inc_14, dec_15, todo, todo, todo, todo, todo, dec_1b, inc_1c, dec_1d,
    todo, todo, // 0x10
    todo, todo, todo, inc_23, inc_24, dec_25, todo, todo, todo, todo, todo, dec_2b, inc_2c, dec_2d,
    todo, todo, // 0x20
    todo, todo, todo, inc_33, inc_34, dec_35, todo, todo, todo, todo, todo, dec_3b, inc_3c, dec_3d,
    todo, todo, // 0x30
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x40
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x50
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x60
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x70
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x80
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0x90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xA0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xB0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xC0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xD0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xE0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,
    todo, // 0xF0
];

fn todo(cpu: &mut Cpu) -> u8 {
    todo!()
}

fn execute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch();
    OPCODES[addr as usize](cpu)
}

fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::BC);
    2
}

fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::B);
    1
}

fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::B);
    1
}

fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::DE);
    2
}

fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::D);
    1
}

fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::D);
    1
}

fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::HL);
    2
}

fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::H);
    1
}

fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::H);
    1
}

fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::SP);
    2
}

fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::HL);
    3
}

fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::HL);
    3
}

fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::BC);
    2
}

fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::C);
    1
}

fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::C);
    1
}

fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::DE);
    2
}

fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::E);
    1
}

fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::E);
    1
}

fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::HL);
    2
}

fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::L);
    1
}

fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::L);
    1
}

fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::SP);
    2
}

fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_u8(Register::A);
    1
}

fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_u8(Register::A);
    1
}
