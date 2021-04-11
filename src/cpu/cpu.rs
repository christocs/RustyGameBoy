mod registers;
use registers::Registers;

mod instruction;
use instruction::{ArithmeticTarget, Instruction};

struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* @todo more targets */ }
                }
            }
            _ => { /* @todo more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_value(value);
        // @todo set flags
        new_value
    }
}
