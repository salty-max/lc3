use std::{
    io::{self, Write},
    process,
};

use super::vm::VM;

#[derive(Debug)]
pub enum OpCode {
    BR,   // branch
    ADD,  // add
    LD,   // load
    ST,   // store
    JSR,  // jump register
    AND,  // and
    LDR,  // load register
    STR,  // store register
    RTI,  // unused
    NOT,  // not
    LDI,  // load indirect
    STI,  // store indirect
    JMP,  // jump
    RES,  // reserved (unused)
    LEA,  // load effective address
    TRAP, // execute trap
}

pub enum TrapCode {
    GETC = 0x20,  // get character from keyboard, not echoed onto the terminal
    OUT = 0x21,   // output a character
    PUTS = 0x22,  // output a word string
    IN = 0x23,    // get character from keyboard, echoed onto the terminal
    PUTSP = 0x24, // output a byte string
    HALT = 0x25,  // halt the program
}

pub fn get_op_code(instruction: &u16) -> Option<OpCode> {
    match instruction >> 12 {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None,
    }
}

pub fn execute_instruction(vm: &mut VM, instr: u16) {
    // Exract opcode
    let op_code = get_op_code(&instr);

    // Match opcode and execute instruction
    match op_code {
        // Some(OpCode::ADD) => add(vm, instr),
        // Some(OpCode::AND) => and(vm, instr),
        // Some(OpCode::BR) => br(vm, instr),
        // Some(OpCode::JMP) => jmp(vm, instr),
        // Some(OpCode::JSR) => jsr(vm, instr),
        // Some(OpCode::LD) => ld(vm, instr),
        // Some(OpCode::LDI) => ldi(vm, instr),
        // Some(OpCode::LDR) => ldr(vm, instr),
        Some(OpCode::LEA) => lea(vm, instr),
        // Some(OpCode::NOT) => not(vm, instr),
        // Some(OpCode::RES) => res(vm, instr),
        // Some(OpCode::RTI) => rti(vm, instr),
        // Some(OpCode::ST) => st(vm, instr),
        // Some(OpCode::STI) => sti(vm, instr),
        // Some(OpCode::STR) => str(vm, instr),
        Some(OpCode::TRAP) => trap(vm, instr),
        _ => {}
    }
}

fn lea(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val = vm.registers.pc + pc_offset;
    vm.registers.update(dr, val);
    vm.registers.update_r_cond_register(dr);
}

fn trap(vm: &mut VM, instr: u16) {
    println!("trap instruction: {:#018b}\n", instr);

    match instr & 0xFF {
        0x20 => {
            // Get character
        }
        0x21 => {
            // Output character
        }
        0x22 => {
            // Puts
            let mut index = vm.registers.r0;
            let mut c = vm.read_memory(index);

            while c != 0x0000 {
                print!("{}", c as u8 as char);
                index += 1;
                c = vm.read_memory(index);
            }

            io::stdout().flush().expect("failed to flush");
        }
        0x23 => {
            // In, print a prompt to the screen and read a single character
        }
        0x24 => {
            // Putsp
        }
        0x25 => {
            // Halt
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
            process::exit(0);
        }
        _ => process::exit(1),
    }
}

fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count
    }

    x
}
