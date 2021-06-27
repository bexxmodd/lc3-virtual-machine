mod lc3;

use crate::lc3::*;

use std::{env, process};

struct Cli {
    args: Vec<String>,
}

fn main() {
    // Load ARguments
    // Setup

    let cli = Cli { args: env::args().collect(), };

    if cli.args.len() < 2 {
        println!("lc3 [image-file1 ...");
        process::exit(2);
    }

    for i in cli.args.iter() {
        if !read_image(i) {
            println!("failed to load image: {}", i);
            process::exit(1);
        }
    }

    let mut memory = Memory::new();
    let mut registers = Regs::new();

    loop {
        // Fetch
        let instr = memory.mem_read(registers.reg[Register::RPC as usize]) as u16;

        // get the opcode
        let op = instr >> 12;

        match op {
            OPAdd => memory.add(instr, &mut registers),
            OPAnd => // AND,
            OPNot => // NOT,
            OPBr => // BR,
            OPJmp => // JMP,
            OPJsr => // JSR,
            OPLd => // LOAD
            OPLdi => memory.ldi(instr, &mut registers),
            OPLdr => // yet another load
            OPLea => // i dont remember
            OPSt => // Store
            OPSti => // another store
            OPStr => // jeez
            OPTrap => // trap
            OPRes => // not available
            OPRti => // not available
            _ => // Invalid OPCODE
        }

        // exit
    }
} 