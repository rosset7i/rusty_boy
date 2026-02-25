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
    ld_0x0a,
    dec_0x0b,
    inc_0x0c,
    dec_0x0d,
    ld_0x0e,
    rrca_0x0f,
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
    ld_0x1a,
    dec_0x1b,
    inc_0x1c,
    dec_0x1d,
    ld_0x1e,
    rra_0x1f,
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
    ld_0x2a,
    dec_0x2b,
    inc_0x2c,
    dec_0x2d,
    ld_0x2e,
    cpl_0x2f,
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
    ld_0x3a,
    dec_0x3b,
    inc_0x3c,
    dec_0x3d,
    ld_0x3e,
    ccf_0x3f,
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
    ld_0x4a,
    ld_0x4b,
    ld_0x4c,
    ld_0x4d,
    ld_0x4e,
    ld_0x4f,
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
    ld_0x5a,
    ld_0x5b,
    ld_0x5c,
    ld_0x5d,
    ld_0x5e,
    ld_0x5f,
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
    ld_0x6a,
    ld_0x6b,
    ld_0x6c,
    ld_0x6d,
    ld_0x6e,
    ld_0x6f,
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
    ld_0x7a,
    ld_0x7b,
    ld_0x7c,
    ld_0x7d,
    ld_0x7e,
    ld_0x7f,
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
    adc_0x8a,
    adc_0x8b,
    adc_0x8c,
    adc_0x8d,
    adc_0x8e,
    adc_0x8f,
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
    sbc_0x9a,
    sbc_0x9b,
    sbc_0x9c,
    sbc_0x9d,
    sbc_0x9e,
    sbc_0x9f,
    and_0xa0,
    and_0xa1,
    and_0xa2,
    and_0xa3,
    and_0xa4,
    and_0xa5,
    and_0xa6,
    and_0xa7,
    xor_0xa8,
    xor_0xa9,
    xor_0xaa,
    xor_0xab,
    xor_0xac,
    xor_0xad,
    xor_0xae,
    xor_0xaf,
    or_0xb0,
    or_0xb1,
    or_0xb2,
    or_0xb3,
    or_0xb4,
    or_0xb5,
    or_0xb6,
    or_0xb7,
    cp_0xb8,
    cp_0xb9,
    cp_0xba,
    cp_0xbb,
    cp_0xbc,
    cp_0xbd,
    cp_0xbe,
    cp_0xbf,
    ret_0xc0,
    pop_0xc1,
    jp_0xc2,
    jp_0xc3,
    call_0xc4,
    push_0xc5,
    add_0xc6,
    rst_0xc7,
    ret_0xc8,
    ret_0xc9,
    jp_0xca,
    prefix_0xcb,
    call_0xcc,
    call_0xcd,
    adc_0xce,
    rst_0xcf,
    ret_0xd0,
    pop_0xd1,
    jp_0xd2,
    illegal_d3_0xd3,
    call_0xd4,
    push_0xd5,
    sub_0xd6,
    rst_0xd7,
    ret_0xd8,
    reti_0xd9,
    jp_0xda,
    illegal_db_0xdb,
    call_0xdc,
    illegal_dd_0xdd,
    sbc_0xde,
    rst_0xdf,
    ldh_0xe0,
    pop_0xe1,
    ldh_0xe2,
    illegal_e3_0xe3,
    illegal_e4_0xe4,
    push_0xe5,
    and_0xe6,
    rst_0xe7,
    add_0xe8,
    jp_0xe9,
    ld_0xea,
    illegal_eb_0xeb,
    illegal_ec_0xec,
    illegal_ed_0xed,
    xor_0xee,
    rst_0xef,
    ldh_0xf0,
    pop_0xf1,
    ldh_0xf2,
    di_0xf3,
    illegal_f4_0xf4,
    push_0xf5,
    or_0xf6,
    rst_0xf7,
    ld_0xf8,
    ld_0xf9,
    ld_0xfa,
    ei_0xfb,
    illegal_fc_0xfc,
    illegal_fd_0xfd,
    cp_0xfe,
    rst_0xff,
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

fn ld_0x0a(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::BC);
    2
}

fn inc_0x0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::C);
    1
}

fn dec_0x0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::C);
    1
}

fn ld_0x0e(cpu: &mut Cpu) -> u8 {
    2
}

fn rrca_0x0f(cpu: &mut Cpu) -> u8 {
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

fn ld_0x1a(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::DE);
    2
}

fn inc_0x1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::E);
    1
}

fn dec_0x1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::E);
    1
}

fn ld_0x1e(cpu: &mut Cpu) -> u8 {
    2
}

fn rra_0x1f(cpu: &mut Cpu) -> u8 {
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

fn ld_0x2a(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::HL);
    2
}

fn inc_0x2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::L);
    1
}

fn dec_0x2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::L);
    1
}

fn ld_0x2e(cpu: &mut Cpu) -> u8 {
    2
}

fn cpl_0x2f(cpu: &mut Cpu) -> u8 {
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

fn ld_0x3a(cpu: &mut Cpu) -> u8 {
    2
}

fn dec_0x3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Register16::SP);
    2
}

fn inc_0x3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Register::A);
    1
}

fn dec_0x3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Register::A);
    1
}

fn ld_0x3e(cpu: &mut Cpu) -> u8 {
    2
}

fn ccf_0x3f(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0x40(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x41(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x42(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x43(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x44(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x45(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x46(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x47(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::B, val);
    1
}

fn ld_0x48(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x49(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x4a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x4b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x4c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x4d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x4e(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x4f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::C, val);
    1
}

fn ld_0x50(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x51(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x52(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x53(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x54(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x55(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x56(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x57(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::D, val);
    1
}

fn ld_0x58(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x59(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x5a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x5b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x5c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x5d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x5e(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x5f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::E, val);
    1
}

fn ld_0x60(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x61(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x62(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x63(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x64(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x65(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x66(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x67(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::H, val);
    1
}

fn ld_0x68(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x69(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x6a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x6b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x6c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x6d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::L, val);
    1
}

fn ld_0x6e(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x6f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::L, val);
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
    let val = cpu.get_u8(Register::B);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x79(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::C);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x7a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::D);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x7b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::E);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x7c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::H);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x7d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::L);
    cpu.set_u8(Register::A, val);
    1
}

fn ld_0x7e(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0x7f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_u8(Register::A);
    cpu.set_u8(Register::A, val);
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

fn adc_0x8a(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8b(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8c(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8d(cpu: &mut Cpu) -> u8 {
    1
}

fn adc_0x8e(cpu: &mut Cpu) -> u8 {
    2
}

fn adc_0x8f(cpu: &mut Cpu) -> u8 {
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

fn sbc_0x9a(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9b(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9c(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9d(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0x9e(cpu: &mut Cpu) -> u8 {
    2
}

fn sbc_0x9f(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa0(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa1(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa2(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa3(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa4(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa5(cpu: &mut Cpu) -> u8 {
    1
}

fn and_0xa6(cpu: &mut Cpu) -> u8 {
    2
}

fn and_0xa7(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xa8(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xa9(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xaa(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xab(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xac(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xad(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xae(cpu: &mut Cpu) -> u8 {
    2
}

fn xor_0xaf(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb0(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb1(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb2(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb3(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb4(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb5(cpu: &mut Cpu) -> u8 {
    1
}

fn or_0xb6(cpu: &mut Cpu) -> u8 {
    2
}

fn or_0xb7(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xb8(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xb9(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xba(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xbb(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xbc(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xbd(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xbe(cpu: &mut Cpu) -> u8 {
    2
}

fn cp_0xbf(cpu: &mut Cpu) -> u8 {
    1
}

fn ret_0xc0(cpu: &mut Cpu) -> u8 {
    5
}

fn pop_0xc1(cpu: &mut Cpu) -> u8 {
    3
}

fn jp_0xc2(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xc3(cpu: &mut Cpu) -> u8 {
    4
}

fn call_0xc4(cpu: &mut Cpu) -> u8 {
    6
}

fn push_0xc5(cpu: &mut Cpu) -> u8 {
    4
}

fn add_0xc6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xc7(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xc8(cpu: &mut Cpu) -> u8 {
    5
}

fn ret_0xc9(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xca(cpu: &mut Cpu) -> u8 {
    4
}

fn prefix_0xcb(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xcc(cpu: &mut Cpu) -> u8 {
    6
}

fn call_0xcd(cpu: &mut Cpu) -> u8 {
    6
}

fn adc_0xce(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xcf(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xd0(cpu: &mut Cpu) -> u8 {
    5
}

fn pop_0xd1(cpu: &mut Cpu) -> u8 {
    3
}

fn jp_0xd2(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_d3_0xd3(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xd4(cpu: &mut Cpu) -> u8 {
    6
}

fn push_0xd5(cpu: &mut Cpu) -> u8 {
    4
}

fn sub_0xd6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xd7(cpu: &mut Cpu) -> u8 {
    4
}

fn ret_0xd8(cpu: &mut Cpu) -> u8 {
    5
}

fn reti_0xd9(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xda(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_db_0xdb(cpu: &mut Cpu) -> u8 {
    1
}

fn call_0xdc(cpu: &mut Cpu) -> u8 {
    6
}

fn illegal_dd_0xdd(cpu: &mut Cpu) -> u8 {
    1
}

fn sbc_0xde(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xdf(cpu: &mut Cpu) -> u8 {
    4
}

fn ldh_0xe0(cpu: &mut Cpu) -> u8 {
    3
}

fn pop_0xe1(cpu: &mut Cpu) -> u8 {
    3
}

fn ldh_0xe2(cpu: &mut Cpu) -> u8 {
    2
}

fn illegal_e3_0xe3(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_e4_0xe4(cpu: &mut Cpu) -> u8 {
    1
}

fn push_0xe5(cpu: &mut Cpu) -> u8 {
    4
}

fn and_0xe6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xe7(cpu: &mut Cpu) -> u8 {
    4
}

fn add_0xe8(cpu: &mut Cpu) -> u8 {
    4
}

fn jp_0xe9(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_0xea(cpu: &mut Cpu) -> u8 {
    4
}

fn illegal_eb_0xeb(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_ec_0xec(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_ed_0xed(cpu: &mut Cpu) -> u8 {
    1
}

fn xor_0xee(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xef(cpu: &mut Cpu) -> u8 {
    4
}

fn ldh_0xf0(cpu: &mut Cpu) -> u8 {
    3
}

fn pop_0xf1(cpu: &mut Cpu) -> u8 {
    3
}

fn ldh_0xf2(cpu: &mut Cpu) -> u8 {
    2
}

fn di_0xf3(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_f4_0xf4(cpu: &mut Cpu) -> u8 {
    1
}

fn push_0xf5(cpu: &mut Cpu) -> u8 {
    4
}

fn or_0xf6(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xf7(cpu: &mut Cpu) -> u8 {
    4
}

fn ld_0xf8(cpu: &mut Cpu) -> u8 {
    3
}

fn ld_0xf9(cpu: &mut Cpu) -> u8 {
    2
}

fn ld_0xfa(cpu: &mut Cpu) -> u8 {
    4
}

fn ei_0xfb(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_fc_0xfc(cpu: &mut Cpu) -> u8 {
    1
}

fn illegal_fd_0xfd(cpu: &mut Cpu) -> u8 {
    1
}

fn cp_0xfe(cpu: &mut Cpu) -> u8 {
    2
}

fn rst_0xff(cpu: &mut Cpu) -> u8 {
    4
}
