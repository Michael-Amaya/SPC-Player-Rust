use crate::models::enums::addressing_modes::AddressingModes;

#[derive(Debug, Default)]
pub struct Instruction {
    pub mnemonic: String,
    pub addressing_mode: AddressingModes,
    pub length: u32,
    pub cycles: u32,
    pub has_conditional_cycles: bool,
}

impl Instruction {
    pub fn build_instruction_array() -> [Self; 256]  {
        let mut instructions = std::array::from_fn(|_| Instruction::default());
        instructions[0x00] = Instruction{ mnemonic: "NOP".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x01] = Instruction{ mnemonic: "BPL".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x02] = Instruction{ mnemonic: "CLRP".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x03] = Instruction{ mnemonic: "BMI".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x04] = Instruction{ mnemonic: "SETP".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x05] = Instruction{ mnemonic: "BVC".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x06] = Instruction{ mnemonic: "CLRC".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x07] = Instruction{ mnemonic: "BVS".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x08] = Instruction{ mnemonic: "SETC".into(), addressing_mode: AddressingModes::Implied, length : 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x09] = Instruction{ mnemonic: "BCC".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x0A] = Instruction{ mnemonic: "EI".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x0B] = Instruction{ mnemonic: "BCS".into(), addressing_mode: AddressingModes::Relative, length : 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x0C] = Instruction{ mnemonic: "DI".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x0D] = Instruction{ mnemonic: "BNE".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x0E] = Instruction{ mnemonic: "CLRV".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x0F] = Instruction{ mnemonic: "BEQ".into(), addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };

        instructions[0x10] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x11] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x12] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x13] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x14] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x15] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x16] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x17] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x18] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x19] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1A] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1B] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1C] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1D] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1E] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1F] = Instruction{ mnemonic: "TCALL".into(), addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };

        

        instructions
    }
}