use crate::models::enums::addressing_modes::AddressingModes;

#[allow(unused)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub addressing_mode: AddressingModes,
    pub length: u32,
    pub cycles: u32,
    pub has_conditional_cycles: bool,
}

impl Instruction {
    pub fn build_instruction_array() -> [Self; 256]  {
        let mut instructions = std::array::from_fn(|_| Instruction::default());

        instructions[0x00] = Instruction{ mnemonic: "NOP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x10] = Instruction{ mnemonic: "BPL", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x20] = Instruction{ mnemonic: "CLRP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x30] = Instruction{ mnemonic: "BMI", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x40] = Instruction{ mnemonic: "SETP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x50] = Instruction{ mnemonic: "BVC", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x60] = Instruction{ mnemonic: "CLRC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x70] = Instruction{ mnemonic: "BVS", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0x80] = Instruction{ mnemonic: "SETC", addressing_mode: AddressingModes::Implied, length : 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x90] = Instruction{ mnemonic: "BCC", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0xA0] = Instruction{ mnemonic: "EI", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xB0] = Instruction{ mnemonic: "BCS", addressing_mode: AddressingModes::Relative, length : 2, cycles: 2, has_conditional_cycles: true };
        instructions[0xC0] = Instruction{ mnemonic: "DI", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xD0] = Instruction{ mnemonic: "BNE", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };
        instructions[0xE0] = Instruction{ mnemonic: "CLRV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xF0] = Instruction{ mnemonic: "BEQ", addressing_mode: AddressingModes::Relative, length: 2, cycles: 2, has_conditional_cycles: true };

        instructions[0x01] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x11] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x21] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x31] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x41] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x51] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x61] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x71] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x81] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x91] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xA1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xB1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xC1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xD1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xE1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0xF1] = Instruction{ mnemonic: "TCALL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };

        instructions[0x02] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x12] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x22] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x32] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x42] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x52] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x62] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x72] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x82] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x92] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xA2] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xB2] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xC2] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xD2] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xE2] = Instruction{ mnemonic: "SET1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xF2] = Instruction{ mnemonic: "CLR1", addressing_mode: AddressingModes::DirectPageBit, length: 2, cycles: 4, has_conditional_cycles: false };

        instructions[0x03] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x13] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x23] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x33] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x43] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x53] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x63] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x73] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x83] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x93] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xA3] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xB3] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xC3] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xD3] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xE3] = Instruction{ mnemonic: "BBS", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0xF3] = Instruction{ mnemonic: "BBC", addressing_mode: AddressingModes::DirectPageBitRelative, length: 3, cycles: 5, has_conditional_cycles: true };

        instructions[0x04] = Instruction{ mnemonic: "OR", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x14] = Instruction{ mnemonic: "OR", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x24] = Instruction{ mnemonic: "AND", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x34] = Instruction{ mnemonic: "AND", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x44] = Instruction{ mnemonic: "EOR", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x54] = Instruction{ mnemonic: "EOR", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x64] = Instruction{ mnemonic: "CMP", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x74] = Instruction{ mnemonic: "CMP", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false }; 
        instructions[0x84] = Instruction{ mnemonic: "ADC", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x94] = Instruction{ mnemonic: "ADC", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xA4] = Instruction{ mnemonic: "SBC", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0xB4] = Instruction{ mnemonic: "SBC", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xC4] = Instruction{ mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xD4] = Instruction{ mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false }; 
        instructions[0xE4] = Instruction{ mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0xF4] = Instruction{ mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };

        instructions[0x05] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x15] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x25] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x35] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x45] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x55] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x65] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x75] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x85] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x95] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xA5] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0xB5] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xC5] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xD5] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0xE5] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0xF5] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };

        instructions[0x06] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x16] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x26] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x36] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x46] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x56] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x66] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x76] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x86] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0x96] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xA6] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xB6] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xC6] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xD6] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0xE6] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::Indirect, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xF6] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::YIndexedAbsolute, length: 3, cycles: 5, has_conditional_cycles: false };

        instructions[0x07] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x17] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x27] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x37] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x47] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x57] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x67] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x77] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x87] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x97] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0xA7] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0xB7] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0xC7] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 7, has_conditional_cycles: false };
        instructions[0xD7] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 7, has_conditional_cycles: false };
        instructions[0xE7] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedIndirect, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0xF7] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::IndirectYIndexed, length: 2, cycles: 6, has_conditional_cycles: false };

        instructions[0x08] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x18] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x28] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x38] = Instruction {mnemonic: "AND", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x48] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x58] = Instruction {mnemonic: "EOR", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x68] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x78] = Instruction {mnemonic: "CMP", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x88] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x98] = Instruction {mnemonic: "ADC", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xA8] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0xB8] = Instruction {mnemonic: "SBC", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xC8] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0xD8] = Instruction {mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xE8] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0xF8] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };

        instructions[0x09] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x19] = Instruction { mnemonic: "OR", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0x29] = Instruction { mnemonic: "AND", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x39] = Instruction { mnemonic: "AND", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0x49] = Instruction { mnemonic: "EOR", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x59] = Instruction { mnemonic: "EOR", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0x69] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x79] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0x89] = Instruction { mnemonic: "ADC", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x99] = Instruction { mnemonic: "ADC", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0xA9] = Instruction { mnemonic: "SBC", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0xB9] = Instruction { mnemonic: "SBC", addressing_mode: AddressingModes::IndirectPageToIndirectPage, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0xC9] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xD9] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::YIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xE9] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0xF9] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::YIndexedDirectPage, length:2, cycles: 4, has_conditional_cycles: false };

        instructions[0x0A] = Instruction { mnemonic: "OR1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x1A] = Instruction { mnemonic: "DECW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x2A] = Instruction { mnemonic: "OR1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x3A] = Instruction { mnemonic: "INCW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x4A] = Instruction { mnemonic: "AND1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x5A] = Instruction { mnemonic: "CMPW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x6A] = Instruction { mnemonic: "AND1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x7A] = Instruction { mnemonic: "ADDW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0x8A] = Instruction { mnemonic: "EOR1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x9A] = Instruction { mnemonic: "SUBW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xAA] = Instruction { mnemonic: "MOV1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0xBA] = Instruction { mnemonic: "MOVW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xCA] = Instruction { mnemonic: "MOV1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0xDA] = Instruction { mnemonic: "MOVW", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xEA] = Instruction { mnemonic: "NOT1", addressing_mode: AddressingModes::AbsoluteBooleanBit, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xFA] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::DirectPageToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };

        instructions[0x0B] = Instruction { mnemonic: "ASL", addressing_mode: AddressingModes::DirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x1B] = Instruction { mnemonic: "ASL", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0x2B] = Instruction { mnemonic: "ROL", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x3B] = Instruction { mnemonic: "ROL", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0x4B] = Instruction { mnemonic: "LSR", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x5B] = Instruction { mnemonic: "LSR", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0x6B] = Instruction { mnemonic: "ROR", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x7B] = Instruction { mnemonic: "ROR", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0x8B] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x9B] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xAB] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xBB] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xCB] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0xDB] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 5, has_conditional_cycles: false };
        instructions[0xEB] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0xFB] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::XIndexedDirectPage, length: 2, cycles: 4, has_conditional_cycles: false };

        instructions[0x0C] = Instruction { mnemonic: "ASL", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x1C] = Instruction { mnemonic: "ASL", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x2C] = Instruction { mnemonic: "ROL", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x3C] = Instruction { mnemonic: "ROL", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x4C] = Instruction { mnemonic: "LSR", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x5C] = Instruction { mnemonic: "LSR", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x6C] = Instruction { mnemonic: "ROR", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x7C] = Instruction { mnemonic: "ROR", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x8C] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x9C] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xAC] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xBC] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xCC] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0xDC] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xEC] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0xFC] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };

        instructions[0x0D] = Instruction { mnemonic: "PUSH", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0x1D] = Instruction { mnemonic: "DEC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x2D] = Instruction { mnemonic: "PUSH", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0x3D] = Instruction { mnemonic: "INC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x4D] = Instruction { mnemonic: "PUSH", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0x5D] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x6D] = Instruction { mnemonic: "PUSH", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0x7D] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0x8D] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0x9D] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xAD] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0xBC] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xCD] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Immediate, length: 2, cycles: 2, has_conditional_cycles: false };
        instructions[0xDD] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };
        instructions[0xED] = Instruction { mnemonic: "NOTC", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xFD] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 2, has_conditional_cycles: false };

        instructions[0x0E] = Instruction { mnemonic: "TSET1", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x1E] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x2E] = Instruction { mnemonic: "CBNE", addressing_mode: AddressingModes::Relative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x3E] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x4E] = Instruction { mnemonic: "TCLR1", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x5E] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 4, has_conditional_cycles: false };
        instructions[0x6E] = Instruction { mnemonic: "DBNZ", addressing_mode: AddressingModes::Relative, length: 3, cycles: 5, has_conditional_cycles: true };
        instructions[0x7E] = Instruction { mnemonic: "CMP", addressing_mode: AddressingModes::DirectPage, length: 2, cycles: 3, has_conditional_cycles: false };
        instructions[0x8E] = Instruction { mnemonic: "POP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0x9E] = Instruction { mnemonic: "DIV", addressing_mode: AddressingModes::Implied, length: 1, cycles: 12, has_conditional_cycles: false };
        instructions[0xAE] = Instruction { mnemonic: "POP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xBE] = Instruction { mnemonic: "DAS", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xCE] = Instruction { mnemonic: "POP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xDE] = Instruction { mnemonic: "CBNE", addressing_mode: AddressingModes::XIndexedDirectPage, length: 3, cycles: 6, has_conditional_cycles: true };
        instructions[0xEE] = Instruction { mnemonic: "POP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xFE] = Instruction { mnemonic: "DBNZ", addressing_mode: AddressingModes::Relative, length: 3, cycles: 5, has_conditional_cycles: true };

        instructions[0x0F] = Instruction { mnemonic: "BRK", addressing_mode: AddressingModes::Implied, length: 1, cycles: 8, has_conditional_cycles: false };
        instructions[0x1F] = Instruction { mnemonic: "JMP", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 6, has_conditional_cycles: false };
        instructions[0x2F] = Instruction { mnemonic: "BRA", addressing_mode: AddressingModes::Relative, length: 2, cycles: 4, has_conditional_cycles: false };
        instructions[0x3F] = Instruction { mnemonic: "CALL", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 8, has_conditional_cycles: false };
        instructions[0x4F] = Instruction { mnemonic: "PCALL", addressing_mode: AddressingModes::Implied, length: 2, cycles: 6, has_conditional_cycles: false };
        instructions[0x5F] = Instruction { mnemonic: "JMP", addressing_mode: AddressingModes::Absolute, length: 3, cycles: 3, has_conditional_cycles: false };
        instructions[0x6F] = Instruction { mnemonic: "RET", addressing_mode: AddressingModes::Implied, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0x7F] = Instruction { mnemonic: "RETI", addressing_mode: AddressingModes::Implied, length: 1, cycles: 6, has_conditional_cycles: false };
        instructions[0x8F] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::ImmediateDataToDirectPage, length: 3, cycles: 5, has_conditional_cycles: false };
        instructions[0x9F] = Instruction { mnemonic: "XCN", addressing_mode: AddressingModes::Accumulator, length: 1, cycles: 5, has_conditional_cycles: false };
        instructions[0xAF] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::IndirectAutoIncrement, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xBF] = Instruction { mnemonic: "MOV", addressing_mode: AddressingModes::IndirectAutoIncrement, length: 1, cycles: 4, has_conditional_cycles: false };
        instructions[0xCF] = Instruction { mnemonic: "MUL", addressing_mode: AddressingModes::Implied, length: 1, cycles: 9, has_conditional_cycles: false };
        instructions[0xDF] = Instruction { mnemonic: "DAA", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xEF] = Instruction { mnemonic: "SLEEP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
        instructions[0xFF] = Instruction { mnemonic: "STOP", addressing_mode: AddressingModes::Implied, length: 1, cycles: 3, has_conditional_cycles: false };
       
       let all_are_valid = instructions.iter().all(|item| !item.mnemonic.is_empty());
       assert!(all_are_valid, "Found an item where mnemonic is empty!");

        instructions
    }
}