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

pub fn get_char() -> u16 {
    // TODO
    1
}

pub struct Regs {
    reg: Vec<u16>,
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            reg: vec![0; Register::RCount as usize],
        }
    }

    pub fn update_flags(&mut self, r: u16) {
        if self.reg[r as usize] == 0 {
            self.reg[Register::RCond as usize] = Flag::BRz as u16;
        } else if (self.reg[r as usize] >> 15) > 0 {
            self.reg[Register::RCond as usize] = Flag::BRn as u16;
        } else {
            self.reg[Register::RCond as usize] = Flag::BRp as u16;
        }
    }

    pub fn update_reg(&mut self, val: u16, index: u16) {
        self.reg[index as usize] = val;
    }

    pub fn get(&self, index: u16) -> u16 {
        self.reg[index as usize]
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

pub trait Directives {
    fn add(&self, instr: u16, reg: &mut Regs);
    fn ldi(&mut self, instr: u16, registers: &mut Regs);
    fn and(&self, instr: u16, registers: &mut Regs);
    fn not(instr: u16);
    fn branch(instr: u16);
    fn jmp(instr: u16);
    fn jsr(instr: u16);
    fn ld(instr: u16);
    fn ldr(instr: u16);
    fn lea(instr: u16);
    fn st(instr: u16);
    fn sti(instr: u16);
}

impl Directives for Memory {
    fn add(&self, instr: u16, registers: &mut Regs) {
        //destination register (DR) 
        let r0 = (instr >> 9) & 0x7;

        // first operand
        let r1 = (instr >> 6) & 0x7;

        // where we are in immediate mode
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5: u16 = sign_extend(instr & 0x1F, 5);
            registers.update_reg(r0, registers.get(r1) + imm5);
        } else {
            let r2 = instr & 0x7;
            registers.update_reg(r0, registers.get(r1) + registers.get(r2));
        }
        registers.update_flags(r0 as u16);
    }

    fn ldi(&mut self, instr: u16, registers: &mut Regs) {
        // destination register (DR)
        let r0 = (instr >> 9) & 0x7;
        
        // PCoffset 9
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        // add pc_offset to the current PC,
        // look at that memory location to get the final address
        registers.update_reg(r0,
            self.mem_read(self.mem_read(
                registers.reg[Register::RPC as usize] + pc_offset))
        );
        registers.update_flags(r0);
    }

    fn and(&self, instr: u16, registers: &mut Regs) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            registers.update_reg(r0, registers.get(r1) & 0x7);
        } else {
            let r2 = instr & 0x7;
            registers.update_reg(r0, registers.get(r1) & registers.get(r2)); 
        }
        registers.update_flags(r0);
    }

    fn not(instr: u16) {
        todo!()
    }

    fn branch(instr: u16) {
        todo!()
    }

    fn jmp(instr: u16) {
        todo!()
    }

    fn jsr(instr: u16) {
        todo!()
    }

    fn ld(instr: u16) {
        todo!()
    }

    fn ldr(instr: u16) {
        todo!()
    }

    fn lea(instr: u16) {
        todo!()
    }

    fn st(instr: u16) {
        todo!()
    }

    fn sti(instr: u16) {
        todo!()
    }
}