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
    
}