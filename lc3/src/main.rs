mod lc3;

use crate::lc3::*;

use std::{env, process};

static mut MEMORY: &'static [u16] = &[0; std::u16::MAX as usize];
static mut REG: &'static [u16] = &[0; Register::RCount as usize];

enum MemoryAddress {
    PCStart = 0x30000,
}

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

    // loop {
    //     // Fetch
    //     let instr = mem_read(reg[RPC]) as u16;

    //     // get the opcode
    //     let op = instr >> 12;

    //     match op {
    //         OPAdd => // ADD,
    //         OPAnd => // AND,
    //         OPNot => // NOT,
    //         OPBr => // BR,
    //         OPJmp => // JMP,
    //         OPJsr => // JSR,
    //         OPLd => // LOAD
    //         OPLDi => // another load,
    //         OPLdr => // yet another load
    //         OPLea => // i dont remember
    //         OPSt => // Store
    //         OPSti => // another store
    //         OPStr => // jeez
    //         OPTrap => // trap
    //         OPRes => // not available
    //         OPRti => // not available
    //         _ => // Invalid OPCODE
    //     }

    //     // exit
    // }
} 