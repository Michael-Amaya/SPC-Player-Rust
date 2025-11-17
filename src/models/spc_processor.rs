use crate::models::{enums::psw_flags::PSWFlags, instructions::Instruction, spc_file::SPCFile};

const SP_START: u16 = 0x0100;

#[derive(Debug)]
pub struct SPCProcessor {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub psw: u8,
    pub sp: u8,

    pub ram_64kb: [u8; 65536],
    pub dsp_registers: [u8; 128],
    pub cycles: u64, // Does it need to be this high?

    instructions: [Instruction; 256],
}

impl SPCProcessor {
    pub fn new(spc_file: &SPCFile) -> Self {
        SPCProcessor {
            pc: u16::from_le_bytes(spc_file.pc),
            a: spc_file.a,
            x: spc_file.x,
            y: spc_file.y,
            psw: spc_file.psw,
            sp: spc_file.sp,
            ram_64kb: spc_file.ram_64kb,
            dsp_registers: spc_file.dsp_registers,
            cycles: 0,
            instructions: Instruction::build_instruction_array(),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.read_ram(self.pc);
        self.pc += 1;

        let instruction = self.instructions[opcode as usize];
        self.execute_instruction(opcode, &instruction);
    }

    fn execute_instruction(&mut self, opcode: u8, instruction: &Instruction) {
        println!("Executing instruction: {:?}", instruction);
        match opcode {
            0x00 => self.op_nop(),
            0x01 => self.op_tcall0(),
            0x02 => self.op_set1(0),
            0x03 => self.op_bbs(0),
            0x04 => {}
            0x05 => {}
            0x06 => {}
            0x07 => {}
            0x08 => {}
            0x09 => {}
            0x0A => {}
            0x0B => {}
            0x0C => {}
            0x0D => {}
            0x0E => {}
            0x0F => {}
            0x10 => self.op_bpl_relative(),
            0x11 => {}
            0x12 => self.op_clr1(0),
            0x13 => self.op_bbc(0),
            0x14 => {}
            0x15 => {}
            0x16 => {}
            0x17 => {}
            0x18 => {}
            0x19 => {}
            0x1A => {}
            0x1B => {}
            0x1C => {}
            0x1D => {}
            0x1E => {}
            0x1F => {}
            0x20 => self.op_clrp(),
            0x21 => {}
            0x22 => self.op_set1(1),
            0x23 => self.op_bbs(1),
            0x24 => {}
            0x25 => {}
            0x26 => {}
            0x27 => {}
            0x28 => {}
            0x29 => {}
            0x2A => {}
            0x2B => {}
            0x2C => {}
            0x2D => {}
            0x2E => {}
            0x2F => {}
            0x30 => {}
            0x31 => {}
            0x32 => self.op_clr1(1),
            0x33 => self.op_bbc(1),
            0x34 => {}
            0x35 => {}
            0x36 => {}
            0x37 => {}
            0x38 => {}
            0x39 => {}
            0x3A => {}
            0x3B => {}
            0x3C => {}
            0x3D => {}
            0x3E => {}
            0x3F => {}
            0x40 => {}
            0x41 => {}
            0x42 => self.op_set1(2),
            0x43 => self.op_bbs(2),
            0x44 => {}
            0x45 => {}
            0x46 => {}
            0x47 => {}
            0x48 => {}
            0x49 => {}
            0x4A => {}
            0x4B => {}
            0x4C => {}
            0x4D => {}
            0x4E => {}
            0x4F => {}
            0x50 => {}
            0x51 => {}
            0x52 => self.op_clr1(2),
            0x53 => self.op_bbc(2),
            0x54 => {}
            0x55 => {}
            0x56 => {}
            0x57 => {}
            0x58 => {}
            0x59 => {}
            0x5A => {}
            0x5B => {}
            0x5C => {}
            0x5D => {}
            0x5E => {}
            0x5F => {}
            0x60 => {}
            0x61 => {}
            0x62 => self.op_set1(3),
            0x63 => self.op_bbs(3),
            0x64 => {}
            0x65 => {}
            0x66 => {}
            0x67 => {}
            0x68 => {}
            0x69 => {}
            0x6A => {}
            0x6B => {}
            0x6C => {}
            0x6D => {}
            0x6E => {}
            0x6F => {}
            0x70 => {}
            0x71 => {}
            0x72 => self.op_clr1(3),
            0x73 => self.op_bbc(3),
            0x74 => {}
            0x75 => {}
            0x76 => {}
            0x77 => {}
            0x78 => {}
            0x79 => {}
            0x7A => {}
            0x7B => {}
            0x7C => {}
            0x7D => {}
            0x7E => {}
            0x7F => {}
            0x80 => {}
            0x81 => {}
            0x82 => self.op_set1(4),
            0x83 => self.op_bbs(4),
            0x84 => {}
            0x85 => {}
            0x86 => {}
            0x87 => {}
            0x88 => {}
            0x89 => {}
            0x8A => {}
            0x8B => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x8F => {}
            0x90 => {}
            0x91 => {}
            0x92 => self.op_clr1(4),
            0x93 => self.op_bbc(4),
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x97 => {}
            0x98 => {}
            0x99 => {}
            0x9A => {}
            0x9B => {}
            0x9C => {}
            0x9D => {}
            0x9E => {}
            0x9F => {}
            0xA0 => {}
            0xA1 => {}
            0xA2 => self.op_set1(5),
            0xA3 => self.op_bbs(5),
            0xA4 => {}
            0xA5 => {}
            0xA6 => {}
            0xA7 => {}
            0xA8 => {}
            0xA9 => {}
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xAF => {}
            0xB0 => {}
            0xB1 => {}
            0xB2 => self.op_clr1(5),
            0xB3 => self.op_bbc(5),
            0xB4 => {}
            0xB5 => {}
            0xB6 => {}
            0xB7 => {}
            0xB8 => {}
            0xB9 => {}
            0xBA => {}
            0xBB => {}
            0xBC => {}
            0xBD => {}
            0xBE => {}
            0xBF => {}
            0xC0 => {}
            0xC1 => {}
            0xC2 => self.op_set1(6),
            0xC3 => self.op_bbs(6),
            0xC4 => {}
            0xC5 => {}
            0xC6 => {}
            0xC7 => {}
            0xC8 => {}
            0xC9 => {}
            0xCA => {}
            0xCB => {}
            0xCC => {}
            0xCD => {}
            0xCE => {}
            0xCF => {}
            0xD0 => {}
            0xD1 => {}
            0xD2 => self.op_clr1(6),
            0xD3 => self.op_bbc(6),
            0xD4 => {}
            0xD5 => {}
            0xD6 => {}
            0xD7 => {}
            0xD8 => {}
            0xD9 => {}
            0xDA => {}
            0xDB => {}
            0xDC => {}
            0xDD => {}
            0xDE => {}
            0xDF => {}
            0xE0 => {}
            0xE1 => {}
            0xE2 => self.op_set1(7),
            0xE3 => self.op_bbs(7),
            0xE4 => {}
            0xE5 => {}
            0xE6 => {}
            0xE7 => {}
            0xE8 => {}
            0xE9 => {}
            0xEA => {}
            0xEB => {}
            0xEC => {}
            0xED => {}
            0xEE => {}
            0xEF => {}
            0xF0 => {}
            0xF1 => {}
            0xF2 => self.op_clr1(7),
            0xF3 => self.op_bbc(7),
            0xF4 => {}
            0xF5 => {}
            0xF6 => {}
            0xF7 => {}
            0xF8 => {}
            0xF9 => {}
            0xFA => {}
            0xFB => {}
            0xFC => {}
            0xFD => {}
            0xFE => {}
            0xFF => {}
        }
    }

    // CPU Operational Functions
    fn read_ram(&self, address: u16) -> u8 {
        // TODO: Have to decide to read from DSP or from RAM....
        self.ram_64kb[address as usize]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        // TODO: Have to decide to read from DSP or from RAM....
        self.ram_64kb[address as usize] = value;
    }

    fn write_stack(&mut self, value: u8) {
        self.write_ram(SP_START + self.sp as u16, value);
        self.sp -= 1;
    }

    fn flag_set(&mut self, flag: PSWFlags) -> bool {
        (self.psw & flag as u8) != 0
    }

    fn set_flag(&mut self, flag: PSWFlags) {
        self.psw |= flag as u8;
    }

    fn clear_flag(&mut self, flag: PSWFlags) {
        self.psw &= !(flag as u8);
    }

    // CPU Opcodes
    // 0x00
    fn op_nop(&mut self) {
        self.cycles += 2;
    }

    // 0x01
    fn op_tcall0(&mut self) {
        let pc_bytes = u16::to_le_bytes(self.pc);
        self.write_stack(pc_bytes[1]);
        self.write_stack(pc_bytes[0]);

        let to_jump = u16::from_le_bytes([self.read_ram(0xFFDE), self.read_ram(0xFFDF)]);
        self.pc = to_jump;
        self.cycles += 8;
    }

    // 0x02, 0x22, 0x42, 0x62, 0x82, 0xA2, 0xC2, 0xE2
    fn op_set1(&mut self, position: u8) {
        assert!(position < 8);

        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + (offset as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value | (1 << position);
        self.write_ram(addr, new_value);

        self.cycles += 4;
    }

    // 0x03, 0x23, 0x43, 0x63, 0x83, 0xA3, 0xC3, 0xE3
    fn op_bbs(&mut self, position: u8) {
        assert!(position < 8);

        let dp_offset = self.read_ram(self.pc);
        self.pc += 1;
        let relative_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + (dp_offset as u16);
        let value = self.read_ram(addr);
        if (value & (1 << position)) != 0 {
            let relative_offset_i8 = relative_offset as i8;
            let relative_offset_i16 = relative_offset_i8 as i16;
            self.pc = self.pc.wrapping_add_signed(relative_offset_i16);
            self.cycles += 7;
        } else {
            self.cycles += 5;
        }
    }

    // 0x12, 0x32, 0x52, 0x72, 0x92, 0xB2, 0xD2, 0xF2
    fn op_clr1(&mut self, position: u8) {
        assert!(position < 8);

        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + (offset as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value & !(1 << position);
        self.write_ram(addr, new_value);

        self.cycles += 4;
    }

    // 0x13, 0x33, 0x53, 0x73, 0x93, 0xB3, 0xD3, 0xF3
    fn op_bbc(&mut self, position: u8) {
        assert!(position < 8);

        let dp_offset = self.read_ram(self.pc);
        self.pc += 1;
        let relative_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + (dp_offset as u16);
        let value = self.read_ram(addr);
        if (value & (1 << position)) == 0 {
            let relative_offset_i8 = relative_offset as i8;
            let relative_offset_i16 = relative_offset_i8 as i16;
            self.pc = self.pc.wrapping_add_signed(relative_offset_i16);
            self.cycles += 7;
        } else {
            self.cycles += 5;
        }
    }

    // 0x10
    fn op_bpl_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if !self.flag_set(PSWFlags::Negative) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0x20
    fn op_clrp(&mut self) {
        self.clear_flag(PSWFlags::DirectPage);
        self.cycles += 2;
    }
}
