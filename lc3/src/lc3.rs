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

// static mut REG: &'static [u16] = &[0; Register::RCount as usize];

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

pub enum MemoryMappedRegisters {
    MrKbsr = 0xFE00, // keyboard status
    MrKbdr = 0xFE02, // Keyboard data
}

pub enum TrapCode {
    TrapGetc = 0x20, //get character from keyboard, not echoed onto the terminal
    TrapOut = 0x21, // output a character
    TrapPuts = 0x22, // output a word string
    TrapIn = 0x23, // get char from keyboard, echoed onto the terminal
    TrapPutsp = 0x24, // output a byte string
    TrapHalt = 0x25, // halt the program
}

pub struct Memory {
    memory: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; std::u16::MAX as usize]
        }
    }

    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    pub fn mem_read(&self, address: u16) -> u16 {
        if address == MemoryMappedRegisters::MrKbsr as u16 {
            if check_key() {
                self.memory[MemoryMappedRegisters::MrKbsr as usize] = 1 << 15;
                self.memory[MemoryMappedRegisters::MrKbsr as usize] = get_char();
            }
        } else {
            self.memory[MemoryMappedRegisters::MrKbsr as usize] = 0;
        }
        self.memory[MemoryMappedRegisters::MrKbsr as usize]
    }
}

pub fn check_key() -> bool {
    false
}

pub struct Regs {
    reg: Vec<u16>,
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            reg: Vec::with_capacity(Register::RCount as usize),
        }
    }
}

enum MemoryAddress {
    PCStart = 0x30000,
}


pub fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    let bit_count = x.count_ones();
    if (x >> (bit_count - 1) & 1) == 1 {
        x |= 0xFFFF << bit_count;
    }
    return x
}

pub fn swap_16bits(x: u16) -> u16 {
    x << 8 | x >> 8
}

pub fn update_flags(r: u16) {
    unsafe {
        if REG[r as usize] == 0 {
            REG[Register::RCond as usize] = Flag::BRz as u16;
        } else if (REG[r as usize] >> 15) > 0 {
            REG[Register::RCond as usize] = Flag::BRn as u16;
        } else {
            REG[Register::RCond as usize] = Flag::BRp as u16;
        }
    }
}

pub fn add_dir(instr: u16) {
    //destination register (DR) 
    let r0 = ((instr >> 9) & 0x7) as usize;

    // first operand
    let r1 = ((instr >> 6) & 0x7) as usize;

    // where we are in immediate mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        unsafe { REG[r0] = REG[r1] + imm5; }
    } else {
        let r2 = (instr & 0x7) as usize;
        unsafe { REG[r0] = REG[r0] + REG[r2]; }
    }

    update_flags(r0 as u16);
}

pub fn ldi_dir(instr: u16) {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;
    
    // PCoffset 9
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // add pc_offset to the current PC,
    // look at that memory location to get the final address
    unsafe {
        REG[r0 as usize] = mem_read(
            mem_read(reg[Register::RPC as usize] + pc_offset)
        )
    };
}