use super::{instruction, register::Registers};

const MEMORY_SIZE: usize = u16::MAX as usize;

pub fn execute_program(vm: &mut VM) {
    while vm.registers.pc < MEMORY_SIZE as u16 {
        // Read instruction
        let instruction = vm.read_memory(vm.registers.pc);

        // Increment pc
        vm.registers.pc += 1;

        // Extract opcode and execute instruction
        instruction::execute_instruction(vm, instruction);
    }
}

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn read_memory(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}
