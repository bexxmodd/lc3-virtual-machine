pub enum Register {
    Rr0 = 0,
    Rr1,
    Rr2,
    Rr3,
    Rr4,
    Rr5,
    Rr6,
    Rr7,
    RPC, // Program Counter
    RCond,
    RCount = 10,
}

static mut REG: &'static [u16] = &[0; Register::RCount as usize];

pub enum Instruction {
    OPBr = 0, // branch
    OPAdd,    // add
    OPLd,     // load
    OPSt,     // store
    OPJsr,    // jump register
    OPAnd,    // bitwise and
    OPLdr,    // load register
    OPStr,    // store register
    OPRti,    // unused
    OPNot,    // bitwise not
    OPLdi,    // load indirect
    OPSti,    // store indirect
    OPJmp,    // jump
    OPRes,    // reserved (unused)
    OPLea,    // load effective address
    OPTrap    // execute trap
}

pub enum Flag {
    BRp = 1 << 0,  // Positive
    BRz = 1 << 1,  // Zero
    BRn = 1 << 2,  // Negative
}

pub fn sign_extend(mut x: u16) -> u16 {
    let bit_count = x.count_ones();
    if x >> (bitCount - 1) & 1 {
        x |= 0xFFFF << bit_count;
    }
    return x
}

pub fn update_flags(r: u16) {
    if REG[r as usize] == 0 {
        REG[RCond as usize] = BRz;
    } else if REG[r as usize] >> 15 {
        REG[RCond as usize] = BRn;
    } else {
        REG[RCond as usize] = BRp;
    }
}

pub fn add_dir(instr: u16) {
    //destination register (DR) 
    let r0 = (instr >> 9) & 0x7;

    // first operand
    let r1 = (instr >> 6) & 0x7;

    // where we are in immediate mode
    let imm_flag = (isntr >> 5) & 0x1;

    if imm_flag {
        let imm5: u16 = sign_extend(instr & 0x1F);
        REG[r0] = REG[r1] + imm5;
    } else {
        let r2: u16 = instr & 0x7;
        REG[r0] = REG[r1] + REG[r2];
    }

    update_flags(r0);
}