use std::fs::File;

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
    RCount,
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

    pub fn write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    pub fn read(&self, address: u16) -> u16 {
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

pub struct Registers {
    reg: Vec<u16>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
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

    pub fn update_reg(&mut self, index: u16, val: u16) {
        self.reg[index as usize] = val;
    }

    pub fn get(&self, index: u16) -> u16 {
        self.reg[index as usize]
    }

    pub fn increment_pc(&mut self, val: u16) {
        self.reg[Register::RPC as usize] += val;
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
    fn add(&self, instr: u16);
    fn ldi(&self, instr: u16, memory: &mut Memory);
    fn and(&self, instr: u16);
    fn not(&self, instr: u16);
    fn branch(&self, instr: u16);
    fn jmp(&self, instr: u16);
    fn jsr(&self, instr: u16);
    fn load(&self, instr: u16, memory: &Memory);
    fn ldr(&self, instr: u16, memory: &Memory);
    fn lea(&self, instr: u16);
    fn store(&self, instr: u16, memory: &mut Memory);
    fn sti(&self, instr: u16, memory: &mut Memory);
    fn str(&self, instr: u16, memory: &mut Memory);
}

impl Directives for Registers {
    fn add(&self, instr: u16) {
        //destination register (DR) 
        let r0 = (instr >> 9) & 0x7;

        // first operand
        let r1 = (instr >> 6) & 0x7;

        // where we are in immediate mode
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5: u16 = sign_extend(instr & 0x1F, 5);
            self.update_reg(r0, self.get(r1) + imm5);
        } else {
            let r2 = instr & 0x7;
            self.update_reg(r0, self.get(r1) + self.get(r2));
        }
        self.update_flags(r0 as u16);
    }

    fn ldi(&self, instr: u16, memory: &mut Memory) {
        // destination register (DR)
        let r0 = (instr >> 9) & 0x7;
        
        // PCoffset 9
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        // add pc_offset to the current PC,
        // look at that memory location to get the final address
        self.update_reg(r0,
            memory.read(memory.read(
                self.get(Register::RPC as u16) + pc_offset))
        );
        self.update_flags(r0);
    }

    fn and(&self, instr: u16) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.update_reg(r0, self.get(r1) & 0x7);
        } else {
            let r2 = instr & 0x7;
            self.update_reg(r0, self.get(r1) & self.get(r2)); 
        }
        self.update_flags(r0);
    }

    fn not(&self, instr: u16) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;

        self.update_reg(r1, !self.get(r1));
        self.update_flags(r0);
    }

    fn branch(&self, instr: u16) {
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let cond_flag = (instr >> 9) & 0x7;

        if cond_flag & self.get(Register::RCond as u16) == 1 {
            self.increment_pc(pc_offset);
        }
    }

    fn jmp(&self, instr: u16) {
        let r1 = (instr >> 6) & 0x7;
        self.update_reg(Register::RPC as u16, r1);
    }

    fn jsr(&self, instr: u16) {
        let long_flag = (instr >> 11) & 1;
        self.update_reg(Register::Rr7 as u16, Register::RPC as u16);
        if long_flag == 1 {
            let long_pc_offset = sign_extend(instr & 0x7FF, 11);
            self.increment_pc(long_pc_offset); // JSR
        } else {
            let r1 = (instr >> 6) & 0x7;
            self.update_reg(Register::RPC as u16, r1);
        }
    }

    fn load(&self, instr: u16, memory: &Memory) {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        self.update_reg(r0, memory.read(
            self.get(Register::RPC as u16) + pc_offset
        ));
        self.update_flags(r0);
    }

    fn ldr(&self, instr: u16, memory: &Memory) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        self.update_reg(r0,
            memory.read(self.get(r1) + offset)
        );
        self.update_flags(r0);
    }

    fn lea(&self, instr: u16) {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        self.update_reg(r0,
            self.get(Register::RPC as u16) + pc_offset);
        self.update_flags(r0);
    }

    fn store(&self, instr: u16, memory: &mut Memory) {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        memory.write(
            self.get(Register::RPC as u16) + pc_offset,
            self.get(r0)
        );
    }

    fn sti(&self, instr: u16, memory: &mut Memory) {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        memory.write(
            memory.read(self.get(Register::RPC as u16) + pc_offset),
            self.get(r0)
        );
    }

    fn str(&self, instr: u16, memory: &mut Memory) {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        memory.write(self.get(r1) + offset,
                self.get(r0));
    }
}

pub fn puts() {
    todo!();
}

pub fn getc() {
    todo!();
}

pub fn read_image_file(file: Vec<u8>) {
    let max_read = u16::MAX;
    
}

pub fn read_image(path: &str) -> Result<(), std::io::Error> {
    let file = std::fs::read(path)?;
    read_image_file(file);
    Ok(())
}