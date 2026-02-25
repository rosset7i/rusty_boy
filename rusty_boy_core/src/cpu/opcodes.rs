use crate::cpu::{Cpu, Register, Register16};

fn execute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch();
    OPCODES[addr as usize](cpu)
}

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    nop_0x00,
    ld_0x01,
    ld_0x02,
    inc_0x03,
    inc_0x04,
    dec_0x05,
    ld_0x06,
    rlca_0x07,
    ld_0x08,
    add_0x09,
    ld_0x0A,
    dec_0x0B,
    inc_0x0C,
    dec_0x0D,
    ld_0x0E,
    rrca_0x0F,
    stop_0x10,
    ld_0x11,
    ld_0x12,
    inc_0x13,
    inc_0x14,
    dec_0x15,
    ld_0x16,
    rla_0x17,
    jr_0x18,
    add_0x19,
    ld_0x1A,
    dec_0x1B,
    inc_0x1C,
    dec_0x1D,
    ld_0x1E,
    rra_0x1F,
    jr_0x20,
    ld_0x21,
    ld_0x22,
    inc_0x23,
    inc_0x24,
    dec_0x25,
    ld_0x26,
    daa_0x27,
    jr_0x28,
    add_0x29,
    ld_0x2A,
    dec_0x2B,
    inc_0x2C,
    dec_0x2D,
    ld_0x2E,
    cpl_0x2F,
    jr_0x30,
    ld_0x31,
    ld_0x32,
    inc_0x33,
    inc_0x34,
    dec_0x35,
    ld_0x36,
    scf_0x37,
    jr_0x38,
    add_0x39,
    ld_0x3A,
    dec_0x3B,
    inc_0x3C,
    dec_0x3D,
    ld_0x3E,
    ccf_0x3F,
    ld_0x40,
    ld_0x41,
    ld_0x42,
    ld_0x43,
    ld_0x44,
    ld_0x45,
    ld_0x46,
    ld_0x47,
    ld_0x48,
    ld_0x49,
    ld_0x4A,
    ld_0x4B,
    ld_0x4C,
    ld_0x4D,
    ld_0x4E,
    ld_0x4F,
    ld_0x50,
    ld_0x51,
    ld_0x52,
    ld_0x53,
    ld_0x54,
    ld_0x55,
    ld_0x56,
    ld_0x57,
    ld_0x58,
    ld_0x59,
    ld_0x5A,
    ld_0x5B,
    ld_0x5C,
    ld_0x5D,
    ld_0x5E,
    ld_0x5F,
    ld_0x60,
    ld_0x61,
    ld_0x62,
    ld_0x63,
    ld_0x64,
    ld_0x65,
    ld_0x66,
    ld_0x67,
    ld_0x68,
    ld_0x69,
    ld_0x6A,
    ld_0x6B,
    ld_0x6C,
    ld_0x6D,
    ld_0x6E,
    ld_0x6F,
    ld_0x70,
    ld_0x71,
    ld_0x72,
    ld_0x73,
    ld_0x74,
    ld_0x75,
    halt_0x76,
    ld_0x77,
    ld_0x78,
    ld_0x79,
    ld_0x7A,
    ld_0x7B,
    ld_0x7C,
    ld_0x7D,
    ld_0x7E,
    ld_0x7F,
    add_0x80,
    add_0x81,
    add_0x82,
    add_0x83,
    add_0x84,
    add_0x85,
    add_0x86,
    add_0x87,
    adc_0x88,
    adc_0x89,
    adc_0x8A,
    adc_0x8B,
    adc_0x8C,
    adc_0x8D,
    adc_0x8E,
    adc_0x8F,
    sub_0x90,
    sub_0x91,
    sub_0x92,
    sub_0x93,
    sub_0x94,
    sub_0x95,
    sub_0x96,
    sub_0x97,
    sbc_0x98,
    sbc_0x99,
    sbc_0x9A,
    sbc_0x9B,
    sbc_0x9C,
    sbc_0x9D,
    sbc_0x9E,
    sbc_0x9F,
    and_0xA0,
    and_0xA1,
    and_0xA2,
    and_0xA3,
    and_0xA4,
    and_0xA5,
    and_0xA6,
    and_0xA7,
    xor_0xA8,
    xor_0xA9,
    xor_0xAA,
    xor_0xAB,
    xor_0xAC,
    xor_0xAD,
    xor_0xAE,
    xor_0xAF,
    or_0xB0,
    or_0xB1,
    or_0xB2,
    or_0xB3,
    or_0xB4,
    or_0xB5,
    or_0xB6,
    or_0xB7,
    cp_0xB8,
    cp_0xB9,
    cp_0xBA,
    cp_0xBB,
    cp_0xBC,
    cp_0xBD,
    cp_0xBE,
    cp_0xBF,
    ret_0xC0,
    pop_0xC1,
    jp_0xC2,
    jp_0xC3,
    call_0xC4,
    push_0xC5,
    add_0xC6,
    rst_0xC7,
    ret_0xC8,
    ret_0xC9,
    jp_0xCA,
    prefix_0xCB,
    call_0xCC,
    call_0xCD,
    adc_0xCE,
    rst_0xCF,
    ret_0xD0,
    pop_0xD1,
    jp_0xD2,
    illegal_d3_0xD3,
    call_0xD4,
    push_0xD5,
    sub_0xD6,
    rst_0xD7,
    ret_0xD8,
    reti_0xD9,
    jp_0xDA,
    illegal_db_0xDB,
    call_0xDC,
    illegal_dd_0xDD,
    sbc_0xDE,
    rst_0xDF,
    ldh_0xE0,
    pop_0xE1,
    ldh_0xE2,
    illegal_e3_0xE3,
    illegal_e4_0xE4,
    push_0xE5,
    and_0xE6,
    rst_0xE7,
    add_0xE8,
    jp_0xE9,
    ld_0xEA,
    illegal_eb_0xEB,
    illegal_ec_0xEC,
    illegal_ed_0xED,
    xor_0xEE,
    rst_0xEF,
    ldh_0xF0,
    pop_0xF1,
    ldh_0xF2,
    di_0xF3,
    illegal_f4_0xF4,
    push_0xF5,
    or_0xF6,
    rst_0xF7,
    ld_0xF8,
    ld_0xF9,
    ld_0xFA,
    ei_0xFB,
    illegal_fc_0xFC,
    illegal_fd_0xFD,
    cp_0xFE,
    rst_0xFF,
];

fn nop_0x00(cpu: &mut Cpu) -> u8 {
    0
}

fn ld_0x01(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x02(cpu: &mut Cpu) -> u8 {
    2
}

fn inc_0x03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::BC);
    2
}

fn inc_0x04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::B);
    1
}

fn dec_0x05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::B);
    1
}

fn ld_0x06(cpu: &mut Cpu) -> u8 {
    2
}

fn rlca_0x07(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x08(cpu: &mut Cpu) -> u8 {
    5
}

fn add_0x09(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x0A(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x0B(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::BC);
    2
}

fn inc_0x0C(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::C);
    1
}

fn dec_0x0D(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::C);
    1
}

fn ld_0x0E(cpu: &mut Cpu) -> u8 {
    2
}

fn rrca_0x0F(cpu: &mut Cpu) -> u8 {
    1
}

fn stop_0x10(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x11(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x12(cpu: &mut Cpu) -> u8 {
    2
}

fn inc_0x13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::DE);
    2
}

fn inc_0x14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::D);
    1
}

fn dec_0x15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::D);
    1
}

fn ld_0x16(cpu: &mut Cpu) -> u8 {
    2
}

fn rla_0x17(cpu: &mut Cpu) -> u8 {
    1
}

fn jr_0x18(cpu: &mut Cpu) -> u8 {
    3
}

fn add_0x19(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x1A(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x1B(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::DE);
    2
}

fn inc_0x1C(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::E);
    1
}

fn dec_0x1D(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::E);
    1
}

fn ld_0x1E(cpu: &mut Cpu) -> u8 {
    2
}

fn rra_0x1F(cpu: &mut Cpu) -> u8 {
    1
}

fn jr_0x20(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x21(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x22(cpu: &mut Cpu) -> u8 {
    2
}

fn inc_0x23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::HL);
    2
}

fn inc_0x24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::H);
    1
}

fn dec_0x25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::H);
    1
}

fn ld_0x26(cpu: &mut Cpu) -> u8 {
    2
}

fn daa_0x27(cpu: &mut Cpu) -> u8 {
    1
}

fn jr_0x28(cpu: &mut Cpu) -> u8 {
    3
}

fn add_0x29(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x2A(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x2B(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::HL);
    2
}

fn inc_0x2C(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::L);
    1
}

fn dec_0x2D(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::L);
    1
}

fn ld_0x2E(cpu: &mut Cpu) -> u8 {
    2
}

fn cpl_0x2F(cpu: &mut Cpu) -> u8 {
    1
}

fn jr_0x30(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x31(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0x32(cpu: &mut Cpu) -> u8 {
    2
}

fn inc_0x33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Register16::SP);
    2
}

fn inc_0x34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::HL);
    3
}

fn dec_0x35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::HL);
    3
}

fn ld_0x36(cpu: &mut Cpu) -> u8 {
    3
}

fn scf_0x37(cpu: &mut Cpu) -> u8 {
    1
}

fn jr_0x38(cpu: &mut Cpu) -> u8 {
    3
}

fn add_0x39(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x3A(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x3B(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::SP);
    2
}

fn inc_0x3C(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::A);
    1
}

fn dec_0x3D(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::A);
    1
}

fn ld_0x3E(cpu: &mut Cpu) -> u8 {
    2
}

fn ccf_0x3F(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x40(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x41(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x42(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x43(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x44(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x45(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x46(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x47(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x48(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x49(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x4A(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x4B(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x4C(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x4D(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x4E(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x4F(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x50(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x51(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x52(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x53(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x54(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x55(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x56(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x57(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x58(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x59(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x5A(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x5B(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x5C(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x5D(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x5E(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x5F(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x60(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x61(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x62(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x63(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x64(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x65(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x66(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x67(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x68(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x69(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x6A(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x6B(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x6C(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x6D(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x6E(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x6F(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x70(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x71(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x72(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x73(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x74(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x75(cpu: &mut Cpu) -> u8 {
    2
}

fn halt_0x76(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x77(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x78(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x79(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x7A(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x7B(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x7C(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x7D(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x7E(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x7F(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x80(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x81(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x82(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x83(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x84(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x85(cpu: &mut Cpu) -> u8 {
    1
}

fn add_0x86(cpu: &mut Cpu) -> u8 {
    2
}

fn add_0x87(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x88(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x89(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8A(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8B(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8C(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8D(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8E(cpu: &mut Cpu) -> u8 {
    2
}

fn adc_0x8F(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x90(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x91(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x92(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x93(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x94(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x95(cpu: &mut Cpu) -> u8 {
    1
}

fn sub_0x96(cpu: &mut Cpu) -> u8 {
    2
}

fn sub_0x97(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x98(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x99(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9A(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9B(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9C(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9D(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9E(cpu: &mut Cpu) -> u8 {
    2
}

fn sbc_0x9F(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA0(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA1(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA2(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA3(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA4(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA5(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xA6(cpu: &mut Cpu) -> u8 {
    2
}

fn and_0xA7(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xA8(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xA9(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xAA(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xAB(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xAC(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xAD(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xAE(cpu: &mut Cpu) -> u8 {
    2
}

fn xor_0xAF(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB0(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB1(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB2(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB3(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB4(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB5(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xB6(cpu: &mut Cpu) -> u8 {
    2
}

fn or_0xB7(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xB8(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xB9(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xBA(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xBB(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xBC(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xBD(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xBE(cpu: &mut Cpu) -> u8 {
    2
}

fn cp_0xBF(cpu: &mut Cpu) -> u8 {
    1
}

fn ret_0xC0(cpu: &mut Cpu) -> u8 {
    5
}

fn pop_0xC1(cpu: &mut Cpu) -> u8 {
    3
}

fn jp_0xC2(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xC3(cpu: &mut Cpu) -> u8 {
    4
}

fn call_0xC4(cpu: &mut Cpu) -> u8 {
    6
}

fn push_0xC5(cpu: &mut Cpu) -> u8 {
    4
}

fn add_0xC6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xC7(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xC8(cpu: &mut Cpu) -> u8 {
    5
}

fn ret_0xC9(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xCA(cpu: &mut Cpu) -> u8 {
    4
}

fn prefix_0xCB(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xCC(cpu: &mut Cpu) -> u8 {
    6
}

fn call_0xCD(cpu: &mut Cpu) -> u8 {
    6
}

fn adc_0xCE(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xCF(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xD0(cpu: &mut Cpu) -> u8 {
    5
}

fn pop_0xD1(cpu: &mut Cpu) -> u8 {
    3
}

fn jp_0xD2(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_d3_0xD3(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xD4(cpu: &mut Cpu) -> u8 {
    6
}

fn push_0xD5(cpu: &mut Cpu) -> u8 {
    4
}

fn sub_0xD6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xD7(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xD8(cpu: &mut Cpu) -> u8 {
    5
}

fn reti_0xD9(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xDA(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_db_0xDB(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xDC(cpu: &mut Cpu) -> u8 {
    6
}

fn illegal_dd_0xDD(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0xDE(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xDF(cpu: &mut Cpu) -> u8 {
    4
}

fn ldh_0xE0(cpu: &mut Cpu) -> u8 {
    3
}

fn pop_0xE1(cpu: &mut Cpu) -> u8 {
    3
}

fn ldh_0xE2(cpu: &mut Cpu) -> u8 {
    2
}

fn illegal_e3_0xE3(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_e4_0xE4(cpu: &mut Cpu) -> u8 {
    1
}

fn push_0xE5(cpu: &mut Cpu) -> u8 {
    4
}

fn and_0xE6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xE7(cpu: &mut Cpu) -> u8 {
    4
}

fn add_0xE8(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xE9(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0xEA(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_eb_0xEB(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_ec_0xEC(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_ed_0xED(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xEE(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xEF(cpu: &mut Cpu) -> u8 {
    4
}

fn ldh_0xF0(cpu: &mut Cpu) -> u8 {
    3
}

fn pop_0xF1(cpu: &mut Cpu) -> u8 {
    3
}

fn ldh_0xF2(cpu: &mut Cpu) -> u8 {
    2
}

fn di_0xF3(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_f4_0xF4(cpu: &mut Cpu) -> u8 {
    1
}

fn push_0xF5(cpu: &mut Cpu) -> u8 {
    4
}

fn or_0xF6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xF7(cpu: &mut Cpu) -> u8 {
    4
}

fn ld_0xF8(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0xF9(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0xFA(cpu: &mut Cpu) -> u8 {
    4
}

fn ei_0xFB(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_fc_0xFC(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_fd_0xFD(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xFE(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xFF(cpu: &mut Cpu) -> u8 {
    4
}
