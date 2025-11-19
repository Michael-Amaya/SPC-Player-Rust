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
            0x01 => self.op_tcall(0xFFDE),
            0x02 => self.op_set1(0),
            0x03 => self.op_bbs_direct_page(0),
            0x04 => self.op_or_direct_page(),
            0x05 => self.op_or_absolute(),
            0x06 => self.op_or_indirect(),
            0x07 => self.op_or_x_indexed_indirect(),
            0x08 => self.op_or_immediate(),
            0x09 => self.op_or_dp_dp(), // TODO: Make sure this works properly..
            0x0A => self.op_or1_absolute_boolean_bit(),
            0x0B => {}
            0x0C => {}
            0x0D => {}
            0x0E => {}
            0x0F => {}
            0x10 => self.op_bpl_relative(),
            0x11 => self.op_tcall(0xFFDC),
            0x12 => self.op_clr1(0),
            0x13 => self.op_bbc_direct_page(0),
            0x14 => self.op_or_x_indexed_direct(),
            0x15 => self.op_or_x_indexed_absolute(),
            0x16 => self.op_or_y_indexed_absolute(),
            0x17 => self.op_or_indirect_y_indexed(),
            0x18 => self.op_or_immediate_to_direct_page(),
            0x19 => self.op_or_indirect_to_indirect(),
            0x1A => {}
            0x1B => {}
            0x1C => {}
            0x1D => {}
            0x1E => {}
            0x1F => {}
            0x20 => self.op_clrp(),
            0x21 => self.op_tcall(0xFFDA),
            0x22 => self.op_set1(1),
            0x23 => self.op_bbs_direct_page(1),
            0x24 => self.op_and_direct_page(),
            0x25 => self.op_and_absolute(),
            0x26 => self.op_and_indirect(),
            0x27 => self.op_and_x_indexed_indirect(),
            0x28 => self.op_and_immediate(),
            0x29 => self.op_and_dp_dp(),
            0x2A => {}
            0x2B => {}
            0x2C => {}
            0x2D => {}
            0x2E => {}
            0x2F => {}
            0x30 => self.op_bmi_relative(),
            0x31 => self.op_tcall(0xFFD8),
            0x32 => self.op_clr1(1),
            0x33 => self.op_bbc_direct_page(1),
            0x34 => self.op_and_x_indexed_direct(),
            0x35 => self.op_and_x_indexed_absolute(),
            0x36 => self.op_and_y_indexed_absolute(),
            0x37 => self.op_and_indirect_y_indexed(),
            0x38 => self.op_and_immediate_to_direct_page(),
            0x39 => self.op_and_indirect_to_indirect(),
            0x3A => {}
            0x3B => {}
            0x3C => {}
            0x3D => {}
            0x3E => {}
            0x3F => {}
            0x40 => self.op_setp(),
            0x41 => self.op_tcall(0xFFD6),
            0x42 => self.op_set1(2),
            0x43 => self.op_bbs_direct_page(2),
            0x44 => self.op_eor_direct_page(),
            0x45 => self.op_eor_absolute(),
            0x46 => self.op_eor_indirect(),
            0x47 => self.op_eor_x_indexed_indirect(),
            0x48 => self.op_eor_immediate(),
            0x49 => self.op_eor_dp_dp(),
            0x4A => {}
            0x4B => {}
            0x4C => {}
            0x4D => {}
            0x4E => {}
            0x4F => {}
            0x50 => self.op_bvc_relative(),
            0x51 => self.op_tcall(0xFFD4),
            0x52 => self.op_clr1(2),
            0x53 => self.op_bbc_direct_page(2),
            0x54 => self.op_eor_x_indexed_direct(),
            0x55 => self.op_eor_x_indexed_absolute(),
            0x56 => self.op_eor_y_indexed_absolute(),
            0x57 => self.op_eor_indirect_y_indexed(),
            0x58 => self.op_eor_immediate_to_direct_page(),
            0x59 => self.op_eor_indirect_to_indirect(),
            0x5A => {}
            0x5B => {}
            0x5C => {}
            0x5D => {}
            0x5E => {}
            0x5F => {}
            0x60 => self.op_clrc(),
            0x61 => self.op_tcall(0xFFD2),
            0x62 => self.op_set1(3),
            0x63 => self.op_bbs_direct_page(3),
            0x64 => self.op_cmp_direct_page(),
            0x65 => self.op_cmp_absolute(),
            0x66 => self.op_cmp_indirect(),
            0x67 => self.op_cmp_x_indexed_indirect(),
            0x68 => self.op_cmp_immediate(),
            0x69 => self.op_cmp_dp_dp(),
            0x6A => {}
            0x6B => {}
            0x6C => {}
            0x6D => {}
            0x6E => {}
            0x6F => {}
            0x70 => self.op_bvs_relative(),
            0x71 => self.op_tcall(0xFFD0),
            0x72 => self.op_clr1(3),
            0x73 => self.op_bbc_direct_page(3),
            0x74 => self.op_cmp_x_indexed_direct(),
            0x75 => self.op_cmp_x_indexed_absolute(),
            0x76 => self.op_cmp_y_indexed_absolute(),
            0x77 => self.op_cmp_indirect_y_indexed(),
            0x78 => self.op_cmp_immediate_to_direct_page(),
            0x79 => self.op_cmp_indirect_to_indirect(),
            0x7A => {}
            0x7B => {}
            0x7C => {}
            0x7D => {}
            0x7E => {}
            0x7F => {}
            0x80 => self.op_setc(),
            0x81 => self.op_tcall(0xFFCE),
            0x82 => self.op_set1(4),
            0x83 => self.op_bbs_direct_page(4),
            0x84 => self.op_adc_direct_page(),
            0x85 => self.op_adc_absolute(),
            0x86 => self.op_adc_indirect(),
            0x87 => self.op_adc_x_indexed_indirect(),
            0x88 => self.op_adc_immediate(),
            0x89 => self.op_abc_dp_dp(),
            0x8A => {}
            0x8B => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x8F => {}
            0x90 => self.op_bcc_relative(),
            0x91 => self.op_tcall(0xFFCC),
            0x92 => self.op_clr1(4),
            0x93 => self.op_bbc_direct_page(4),
            0x94 => self.op_adc_x_indexed_direct(),
            0x95 => self.op_adc_x_indexed_absolute(),
            0x96 => self.op_adc_y_indexed_absolute(),
            0x97 => self.op_adc_indirect_y_indexed(),
            0x98 => self.op_adc_immediate_to_direct_page(),
            0x99 => self.op_adc_indirect_to_indirect(),
            0x9A => {}
            0x9B => {}
            0x9C => {}
            0x9D => {}
            0x9E => {}
            0x9F => {}
            0xA0 => self.op_ei(),
            0xA1 => self.op_tcall(0xFFCA),
            0xA2 => self.op_set1(5),
            0xA3 => self.op_bbs_direct_page(5),
            0xA4 => self.op_sbc_direct_page(),
            0xA5 => self.op_sbc_absolute(),
            0xA6 => self.op_sbc_indirect(),
            0xA7 => self.op_sbc_x_indexed_indirect(),
            0xA8 => self.op_sbc_immediate(),
            0xA9 => self.op_sbc_dp_dp(),
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xAF => {}
            0xB0 => self.op_bcs_relative(),
            0xB1 => self.op_tcall(0xFFC8),
            0xB2 => self.op_clr1(5),
            0xB3 => self.op_bbc_direct_page(5),
            0xB4 => self.op_sbc_x_indexed_direct(),
            0xB5 => self.op_sbc_x_indexed_absolute(),
            0xB6 => self.op_sbc_y_indexed_absolute(),
            0xB7 => self.op_sbc_indirect_y_indexed(),
            0xB8 => self.op_sbc_immediate_to_direct_page(),
            0xB9 => self.op_sbc_indirect_to_indirect(),
            0xBA => {}
            0xBB => {}
            0xBC => {}
            0xBD => {}
            0xBE => {}
            0xBF => {}
            0xC0 => self.op_di(),
            0xC1 => self.op_tcall(0xFFC6),
            0xC2 => self.op_set1(6),
            0xC3 => self.op_bbs_direct_page(6),
            0xC4 => self.op_mov_direct_page(),
            0xC5 => self.op_mov_absolute(),
            0xC6 => self.op_mov_indirect(),
            0xC7 => self.op_mov_x_indexed_indirect(),
            0xC8 => self.op_cmp_x_immediate(),
            0xC9 => self.op_mov_x_to_absolute(),
            0xCA => {}
            0xCB => {}
            0xCC => {}
            0xCD => {}
            0xCE => {}
            0xCF => {}
            0xD0 => self.op_bne_relative(),
            0xD1 => self.op_tcall(0xFFC4),
            0xD2 => self.op_clr1(6),
            0xD3 => self.op_bbc_direct_page(6),
            0xD4 => self.op_mov_x_indexed_direct(),
            0xD5 => self.op_mov_x_indexed_absolute(),
            0xD6 => self.op_mov_y_indexed_absolute(),
            0xD7 => self.op_mov_indirect_y_indexed(),
            0xD8 => self.op_mov_x_to_direct_page(),
            0xD9 => self.op_mov_y_indexed_direct_page(),
            0xDA => {}
            0xDB => {}
            0xDC => {}
            0xDD => {}
            0xDE => {}
            0xDF => {}
            0xE0 => self.op_clrv(),
            0xE1 => self.op_tcall(0xFFC2),
            0xE2 => self.op_set1(7),
            0xE3 => self.op_bbs_direct_page(7),
            0xE4 => self.op_mov2_direct_page(),
            0xE5 => self.op_mov2_absolute(),
            0xE6 => self.op_mov2_indirect(),
            0xE7 => self.op_mov2_x_indexed_indirect(),
            0xE8 => self.op_mov2_immediate(),
            0xE9 => self.op_mov_absolute_to_x(),
            0xEA => {}
            0xEB => {}
            0xEC => {}
            0xED => {}
            0xEE => {}
            0xEF => {}
            0xF0 => self.op_beq_relative(),
            0xF1 => self.op_tcall(0xFFC0),
            0xF2 => self.op_clr1(7),
            0xF3 => self.op_bbc_direct_page(7),
            0xF4 => self.op_mov2_x_indexed_direct(),
            0xF5 => self.op_mov2_x_indexed_absolute(),
            0xF6 => self.op_mov2_y_indexed_absolute(),
            0xF7 => self.op_mov2_indirect_y_indexed(),
            0xF8 => self.op_mov2_direct_page_to_x(),
            0xF9 => self.op_mov2_y_indexed_direct_page(),
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

    fn update_nz(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.set_flag(PSWFlags::Negative);
        } else {
            self.clear_flag(PSWFlags::Negative);
        }

        if value == 0 {
            self.set_flag(PSWFlags::Zero);
        } else {
            self.clear_flag(PSWFlags::Zero);
        }
    }

    fn update_carry(&mut self, did_borrow: bool) {
        if did_borrow {
            self.clear_flag(PSWFlags::Carry);
        } else {
            self.set_flag(PSWFlags::Carry);
        }
    }

    fn update_overflow(&mut self, to_check: bool) {
        if to_check {
            self.set_flag(PSWFlags::Overflow);
        } else {
            self.clear_flag(PSWFlags::Overflow);
        }
    }

    fn update_half_carry(&mut self, to_check: bool) {
        if to_check {
            self.set_flag(PSWFlags::HalfCarry);
        } else {
            self.clear_flag(PSWFlags::HalfCarry);
        }
    }

    // CPU Opcodes
    // 0x00
    fn op_nop(&mut self) {
        self.cycles += 2;
    }

    // 0x01, 0x11, 0x21, 0x31, 0x41, 0x51, 0x61, 0x71, 0x81, 0x91, 0xA1, 0xB1, 0xC1, 0xD1, 0xE1, 0xF1
    fn op_tcall(&mut self, address: u16) {
        let pc_bytes = u16::to_le_bytes(self.pc);
        self.write_stack(pc_bytes[1]);
        self.write_stack(pc_bytes[0]);

        let to_jump = u16::from_le_bytes([self.read_ram(address), self.read_ram(address + 1)]);
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
    fn op_bbs_direct_page(&mut self, position: u8) {
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

    // 0x04
    fn op_or_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };
        
        let old_value = self.read_ram(addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);

        self.cycles += 3;
    }

    // 0x05
    fn op_or_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let old_value = self.read_ram(addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x06
    fn op_or_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 3;
    }

    // 0x07
    fn op_or_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x08
    fn op_or_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        let new_value = self.a | value;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 2;
    }

    // 0x09
    fn op_or_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let old_value = self.read_ram(dest_addr);
        let new_value = old_value | self.read_ram(source_addr);
        self.write_ram(dest_addr, new_value);
        
        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x0A
    fn op_or1_absolute_boolean_bit(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let packed = self.read_ram(self.pc);
        self.pc += 1;
        let addr = (((packed & 0b0001_1111) as u16) << 8) | addr_low as u16;
        let bit = packed >> 5;

        let value_m = self.read_ram(addr);
        let bit_is_set = ((value_m >> bit) & 1) != 0;
        let carry_is_set = self.flag_set(PSWFlags::Carry);

        if bit_is_set || carry_is_set {
            self.set_flag(PSWFlags::Carry);
        } else {
            self.clear_flag(PSWFlags::Carry);
        }

        self.cycles += 5;
    }

    // 0x0B

    // 0x0C

    // 0x0D

    // 0x0E

    // 0x0F

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
    fn op_bbc_direct_page(&mut self, position: u8) {
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

    // 0x14
    fn op_or_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        let old_value = self.read_ram(addr as u16);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x15
    fn op_or_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.x as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x16
    fn op_or_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x17
    fn op_or_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value | self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x18
    fn op_or_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let old_value = self.read_ram(addr);
        let value = old_value | to_or;
        self.write_ram(addr, value);
        
        self.update_nz(value);
        self.cycles += 5;
    }

    // 0x19
    fn op_or_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        let new_value = first_value | second_value;
        self.write_ram(self.x as u16, new_value);

        self.update_nz(new_value);
        self.cycles += 5; 
    }

    // 0x20
    fn op_clrp(&mut self) {
        self.clear_flag(PSWFlags::DirectPage);
        self.cycles += 2;
    }

    // 0x24
    fn op_and_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };
        
        let old_value = self.read_ram(addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);

        self.cycles += 3;
    }

    // 0x25
    fn op_and_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let old_value = self.read_ram(addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x26
    fn op_and_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 3;
    }

    // 0x27
    fn op_and_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x28
    fn op_and_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        let new_value = self.a & value;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 2;
    }

    // 0x29
    fn op_and_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let old_value = self.read_ram(dest_addr);
        let new_value = old_value & self.read_ram(source_addr);
        self.write_ram(dest_addr, new_value);
        
        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x30
    fn op_bmi_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if self.flag_set(PSWFlags::Negative) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0x34
    fn op_and_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        let old_value = self.read_ram(addr as u16);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x35
    fn op_and_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.x as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x36
    fn op_and_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x37
    fn op_and_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value & self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x38
    fn op_and_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let old_value = self.read_ram(addr);
        let value = old_value & to_or;
        self.write_ram(addr, value);
        
        self.update_nz(value);
        self.cycles += 5;
    }

    // 0x39
    fn op_and_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        let new_value = first_value & second_value;
        self.write_ram(self.x as u16, new_value);

        self.update_nz(new_value);
        self.cycles += 5; 
    }

    // 0x40
    fn op_setp(&mut self) {
        self.set_flag(PSWFlags::DirectPage);
        self.cycles += 2;
    }

    // 0x44
    fn op_eor_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };
        
        let old_value = self.read_ram(addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);

        self.cycles += 3;
    }

    // 0x45
    fn op_eor_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let old_value = self.read_ram(addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x46
    fn op_eor_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 3;
    }

    // 0x47
    fn op_eor_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x48
    fn op_eor_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        let new_value = self.a ^ value;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 2;
    }

    // 0x49
    fn op_eor_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let old_value = self.read_ram(dest_addr);
        let new_value = old_value ^ self.read_ram(source_addr);
        self.write_ram(dest_addr, new_value);
        
        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x50
    fn op_bvc_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if !self.flag_set(PSWFlags::Overflow) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0x54
    fn op_eor_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        let old_value = self.read_ram(addr as u16);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0x55
    fn op_eor_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.x as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x56
    fn op_eor_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        let old_value = self.read_ram(addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x57
    fn op_eor_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let old_value = self.read_ram(final_addr);
        let new_value = old_value ^ self.a;
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x58
    fn op_eor_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let old_value = self.read_ram(addr);
        let value = old_value ^ to_or;
        self.write_ram(addr, value);
        
        self.update_nz(value);
        self.cycles += 5;
    }

    // 0x59
    fn op_eor_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        let new_value = first_value ^ second_value;
        self.write_ram(self.x as u16, new_value);

        self.update_nz(new_value);
        self.cycles += 5; 
    }

    // 0x60
    fn op_clrc(&mut self) {
        self.clear_flag(PSWFlags::Carry);
        self.cycles += 2;
    }

    // 0x64
    fn op_cmp_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };
        
        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        // Update nz flags
        self.update_nz(check_value);
        // Update carry flag
        self.update_carry(did_borrow);

        self.cycles += 3;
    }

    // 0x65
    fn op_cmp_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 4;
    }

    // 0x66
    fn op_cmp_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 3;
    }

    // 0x67
    fn op_cmp_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let old_value = self.read_ram(final_addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 6;
    }

    // 0x68
    fn op_cmp_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        let (check_value, did_borrow) = self.a.overflowing_sub(value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 2;
    }

    // 0x69
    fn op_cmp_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let old_value = self.read_ram(dest_addr);
        let (check_value, did_borrow) = old_value.overflowing_sub(self.read_ram(source_addr));
        
        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 6;
    }

    // 0x70
    fn op_bvs_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if self.flag_set(PSWFlags::Overflow) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0x74
    fn op_cmp_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        let old_value = self.read_ram(addr as u16);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 4;
    }

    // 0x75
    fn op_cmp_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.x as u16);
        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 5;
    }

    // 0x76
    fn op_cmp_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 5;
    }

    // 0x77
    fn op_cmp_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let old_value = self.read_ram(final_addr);
        let (check_value, did_borrow) = self.a.overflowing_sub(old_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 6;
    }

    // 0x78
    fn op_cmp_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let old_value = self.read_ram(addr);
        let (check_value, did_borrow) = old_value.overflowing_sub(to_or);
        
        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 5;
    }

    // 0x79
    fn op_cmp_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        let (check_value, did_borrow) = first_value.overflowing_sub(second_value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 5; 
    }

    // 0x80
    fn op_setc(&mut self) {
        self.set_flag(PSWFlags::Carry);
        self.cycles += 2;
    }

    // 0x84
    fn op_adc_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };

        let a_val = self.a as u16;
        let d_val = self.read_ram(addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + d_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (d_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (d_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 3;
    }

    //0x85
    fn op_adc_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let a_val = self.a as u16;
        let absolute_val = self.read_ram(addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + absolute_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (absolute_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (absolute_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 4;
    }

    // 0x86
    fn op_adc_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let a_val = self.a as u16;
        let indirect_val = old_value as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + indirect_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (indirect_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (indirect_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 3;
    }

    // 0x87
    fn op_adc_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let a_val = self.a as u16;
        let indirect_val = self.read_ram(final_addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + indirect_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (indirect_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (indirect_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 6;
    }

    // 0x88
    fn op_adc_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        
        let a_val = self.a as u16;
        let immediate_val = value as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + immediate_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (immediate_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (immediate_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 2;
    }

    // 0x89
    fn op_abc_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let dest_value = self.read_ram(dest_addr) as u16;
        let source_value = self.read_ram(source_addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let new_value = dest_value + source_value + c_val;
        let new_value_u8 = new_value as u8;
        self.write_ram(dest_addr, new_value_u8);
        
        self.update_nz(new_value_u8);
        self.update_carry(new_value <= 0xFF);
        self.update_half_carry((dest_value & 0x0F) + (source_value & 0x0F) + c_val > 0x0F);
        self.update_overflow(((dest_value ^ new_value) & (source_value ^ new_value)) & 0b1000_0000 != 0);

        self.cycles += 6;
    }


    // 0x90
    fn op_bcc_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if !self.flag_set(PSWFlags::Carry) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0x94
    fn op_adc_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let effective_offset = offset.wrapping_add(self.x);
        let final_addr = base_addr + (effective_offset as u16);
        let offset_val = self.read_ram(final_addr) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 4;
    }

    // 0x95
    fn op_adc_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);
        let final_addr = base_addr.wrapping_add(self.x as u16);

        let offset_val = self.read_ram(final_addr) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 5;
    }

    // 0x96
    fn op_adc_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr = base_addr.wrapping_add(self.y as u16);
        let offset_val = self.read_ram(addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };
        let a_val = self.a as u16;
        
        let full_result = a_val + offset_val + c_val;
        let new_value = full_result as u8;
        self.a = new_value;

        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0x97
    fn op_adc_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let offset_val = self.read_ram(final_addr) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let new_value = full_result as u8;
        self.a = new_value;

        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0x98
    fn op_adc_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let offset_val = self.read_ram(addr) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };
        let to_or_val = to_or as u16;

        let full_result = offset_val + to_or_val + c_val;
        let value = full_result as u8;
        self.write_ram(addr, value);

        self.update_nz(value);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((offset_val & 0x0F) + (to_or_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((offset_val ^ full_result) & (to_or_val ^ full_result)) & 0b1000_0000 != 0);
        
        self.cycles += 5;
    }

    // 0x99
    fn op_adc_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        // let new_value = first_value | second_value;
        // self.write_ram(self.x as u16, new_value);
        let f_val = first_value as u16;
        let s_val = second_value as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = f_val + s_val + c_val;
        let new_value = full_result as u8;
        self.write_ram(self.x as u16, new_value);

        self.update_nz(new_value);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((f_val & 0x0F) + (s_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((f_val ^ full_result) & (s_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 5; 
    }

    // 0xA0
    fn op_ei(&mut self) {
        self.set_flag(PSWFlags::InterruptEnabled);
        self.cycles += 3;
    }

    // 0xA4
    fn op_sbc_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };

        let a_val = self.a as u16;
        let d_val = (self.read_ram(addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + d_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (d_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (d_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 3;
    }

    // 0xA5
    fn op_sbc_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let a_val = self.a as u16;
        let absolute_val = (self.read_ram(addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + absolute_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (absolute_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (absolute_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 4;
    }

    // 0xA6
    fn op_sbc_indirect(&mut self) {
        let addr = self.x as u16;

        let old_value = self.read_ram(addr);
        let a_val = self.a as u16;
        let indirect_val = (old_value ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + indirect_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (indirect_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (indirect_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 3;
    }

    // 0xA7
    fn op_sbc_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let a_val = self.a as u16;
        let indirect_val = (self.read_ram(final_addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + indirect_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (indirect_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (indirect_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 6;
    }

    // 0xA8
    fn op_sbc_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        
        let a_val = self.a as u16;
        let immediate_val = (value ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + immediate_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);

        // Set Half-carry
        self.update_half_carry((a_val & 0x0F) + (immediate_val & 0x0F) + c_val > 0x0F);

        // Set Overflow flag..
        self.update_overflow(((a_val ^ full_result) & (immediate_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 2;
    }

    // 0xA9
    fn op_sbc_dp_dp(&mut self) {
        let ds_offset = self.read_ram(self.pc);
        self.pc += 1;
        let dd_offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let source_addr = base_addr + (ds_offset as u16);
        let dest_addr = base_addr + (dd_offset as u16);

        let dest_value = self.read_ram(dest_addr) as u16;
        let source_value = (self.read_ram(source_addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let new_value = dest_value + source_value + c_val;
        let new_value_u8 = new_value as u8;
        self.write_ram(dest_addr, new_value_u8);
        
        self.update_nz(new_value_u8);
        self.update_carry(new_value <= 0xFF);
        self.update_half_carry((dest_value & 0x0F) + (source_value & 0x0F) + c_val > 0x0F);
        self.update_overflow(((dest_value ^ new_value) & (source_value ^ new_value)) & 0b1000_0000 != 0);

        self.cycles += 6;
    }

    // 0xB0
    fn op_bcs_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if self.flag_set(PSWFlags::Carry) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0xB4
    fn op_sbc_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let effective_offset = offset.wrapping_add(self.x);
        let final_addr = base_addr + (effective_offset as u16);
        let offset_val = (self.read_ram(final_addr) ^ 0xFF) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 4;
    }

    // 0xB5
    fn op_sbc_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);
        let final_addr = base_addr.wrapping_add(self.x as u16);

        let offset_val = (self.read_ram(final_addr) ^ 0xFF) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let full_result_u8 = full_result as u8;
        self.a = full_result_u8;

        self.update_nz(full_result_u8);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);
        self.cycles += 5;
    }

    // 0xB6
    fn op_sbc_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr = base_addr.wrapping_add(self.y as u16);
        let offset_val = (self.read_ram(addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };
        let a_val = self.a as u16;
        
        let full_result = a_val + offset_val + c_val;
        let new_value = full_result as u8;
        self.a = new_value;

        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);

        self.update_nz(new_value);
        self.cycles += 5;
    }
    
    // 0xB7
    fn op_sbc_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let offset_val = (self.read_ram(final_addr) ^ 0xFF) as u16;
        let a_val = self.a as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = a_val + offset_val + c_val;
        let new_value = full_result as u8;
        self.a = new_value;

        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((a_val & 0x0F) + (offset_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((a_val ^ full_result) & (offset_val ^ full_result)) & 0b1000_0000 != 0);

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0xB8
    fn op_sbc_immediate_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let to_or = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr + offset as u16;
        let offset_val = (self.read_ram(addr) ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };
        let to_or_val = to_or as u16;

        let full_result = offset_val + to_or_val + c_val;
        let value = full_result as u8;
        self.write_ram(addr, value);

        self.update_nz(value);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((offset_val & 0x0F) + (to_or_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((offset_val ^ full_result) & (to_or_val ^ full_result)) & 0b1000_0000 != 0);
        
        self.cycles += 5;
    }

    // 0xB9
    fn op_sbc_indirect_to_indirect(&mut self) {
        let first_value = self.read_ram(self.x as u16);
        let second_value = self.read_ram(self.y as u16);

        // let new_value = first_value | second_value;
        // self.write_ram(self.x as u16, new_value);
        let f_val = first_value as u16;
        let s_val = (second_value ^ 0xFF) as u16;
        let c_val = if self.flag_set(PSWFlags::Carry) { 1 } else { 0 };

        let full_result = f_val + s_val + c_val;
        let new_value = full_result as u8;
        self.write_ram(self.x as u16, new_value);

        self.update_nz(new_value);
        self.update_carry(full_result <= 0xFF);
        self.update_half_carry((f_val & 0x0F) + (s_val & 0x0F) + c_val > 0x0F);
        self.update_overflow(((f_val ^ full_result) & (s_val ^ full_result)) & 0b1000_0000 != 0);

        self.cycles += 5; 
    }

    // 0xC0
    fn op_di(&mut self) {
        self.clear_flag(PSWFlags::InterruptEnabled);
        self.cycles += 3;
    }

    // 0xC4
    fn op_mov_direct_page(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };
        let final_addr = base_addr + offset as u16;

        self.write_ram(final_addr as u16, self.a);

        // May not be needed?
        self.update_nz(self.a);
        self.cycles += 4;
    }

    // 0xC5
    fn op_mov_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        self.write_ram(addr, self.a);

        self.update_nz(self.a);
        self.cycles += 5;
    }

    // 0xC6
    fn op_mov_indirect(&mut self) {
        let addr = self.x as u16;
        self.write_ram(addr, self.a);

        self.update_nz(self.a);
        self.cycles += 4;
    }

    // 0xC7
    fn op_mov_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        self.write_ram(final_addr, self.a);

        self.update_nz(self.a);
        self.cycles += 7;
    }

    // 0xC8
    fn op_cmp_x_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        let (check_value, did_borrow) = self.x.overflowing_sub(value);

        self.update_nz(check_value);
        self.update_carry(did_borrow);
        self.cycles += 2;
    }

    // 0xC9
    fn op_mov_x_to_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        self.write_ram(addr, self.x);
        self.update_nz(self.x);
        self.cycles += 5;
    }

    // 0xD0
    fn op_bne_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if !self.flag_set(PSWFlags::Zero) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0xD4
    fn op_mov_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        self.write_ram(addr, self.a);

        self.update_nz(self.a);
        self.cycles += 5;
    }

    // 0xD5
    fn op_mov_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr = base_addr.wrapping_add(self.x as u16);
        self.write_ram(addr, self.a);

        self.update_nz(self.a);
        self.cycles += 6;
    }

    // 0xD6
    fn op_mov_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        self.write_ram(addr, self.a);

        self.update_nz(self.a);
        self.cycles += 6;
    }

    // 0xD7
    fn op_mov_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        self.write_ram(final_addr, self.a);

        self.update_nz(self.a);
        self.cycles += 7;
    }

    // 0xD8
    fn op_mov_x_to_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = base_addr.wrapping_add(offset as u16);
        self.write_ram(addr, self.x);

        self.update_nz(self.x);
        self.cycles += 4;
    }

    // 0xD9
    fn op_mov_y_indexed_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let effective_offset = offset.wrapping_add(self.y);
        let addr = base_addr + (effective_offset as u16);
        self.write_ram(addr, self.x);

        self.update_nz(self.x);
        self.cycles += 5;
    }

    // 0xE0
    fn op_clrv(&mut self) {
        self.clear_flag(PSWFlags::Overflow);
        self.clear_flag(PSWFlags::HalfCarry);
        self.cycles += 2;
    }

    // 0xE4
    fn op_mov2_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let addr = if self.flag_set(PSWFlags::DirectPage) {
            0x0100 + offset as u16
        } else {
            0x0000 + offset as u16
        };
        
        let new_value = self.read_ram(addr);
        self.a = new_value;
        self.update_nz(new_value);

        self.cycles += 3;
    }

    // 0xE5
    fn op_mov2_absolute(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let new_value = self.read_ram(addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0xE6
    fn op_mov2_indirect(&mut self) {
        let addr = self.x as u16;

        let new_value = self.read_ram(addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 3;
    }

    // 0xE7
    fn op_mov2_x_indexed_indirect(&mut self) { 
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        // Dumb pointer logic zzzzzzzzzzzzzz
        let pointer_addr = self.x.wrapping_add(offset);
        let final_addr_low = self.read_ram(pointer_addr as u16);
        let final_addr_high = self.read_ram(pointer_addr.wrapping_add(1) as u16);
        let final_addr = u16::from_le_bytes([final_addr_low, final_addr_high]);

        let new_value = self.read_ram(final_addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0xE8
    fn op_mov2_immediate(&mut self) {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        
        self.a = value;

        self.update_nz(value);
        self.cycles += 2;
    }

    // 0xE9
    fn op_mov_absolute_to_x(&mut self) {
        let addr_low = self.read_ram(self.pc);
        self.pc += 1;
        let addr_high = self.read_ram(self.pc);
        self.pc += 1;
        let addr = u16::from_le_bytes([addr_low, addr_high]);

        let new_value = self.read_ram(addr);
        self.x = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0xF0
    fn op_beq_relative(&mut self) {
        let offset_u8 = self.read_ram(self.pc);
        self.pc += 1;

        if self.flag_set(PSWFlags::Zero) {
            let offset_i8 = offset_u8 as i8;
            let offset_i16 = offset_i8 as i16;
            let new_pc = self.pc.wrapping_add(offset_i16 as u16);
            self.pc = new_pc;
            self.cycles += 4;
        } else {
            self.cycles += 2;
        }
    }

    // 0xF4
    fn op_mov2_x_indexed_direct(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let addr = self.x.wrapping_add(offset as u8) as u16 + base_addr;
        let new_value = self.read_ram(addr as u16);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

    // 0xF5
    fn op_mov2_x_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr = base_addr.wrapping_add(self.x as u16);
        let new_value = self.read_ram(addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0xF6
    fn op_mov2_y_indexed_absolute(&mut self) {
        let low_addr = self.read_ram(self.pc);
        self.pc += 1;
        let high_addr = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr = u16::from_le_bytes([low_addr, high_addr]);

        let addr =base_addr.wrapping_add(self.y as u16);
        let new_value = self.read_ram(addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 5;
    }

    // 0xF7
    fn op_mov2_indirect_y_indexed(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;
        let pointer_addr = offset as u16;
        let base_addr_low = self.read_ram(pointer_addr);
        let base_addr_high = self.read_ram(pointer_addr.wrapping_add(1));
        let base_addr = u16::from_le_bytes([base_addr_low, base_addr_high]);
        let final_addr = base_addr.wrapping_add(self.y as u16);

        let new_value = self.read_ram(final_addr);
        self.a = new_value;

        self.update_nz(new_value);
        self.cycles += 6;
    }

    // 0xF8
    fn op_mov2_direct_page_to_x(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };

        let final_addr = base_addr + offset as u16;
        let new_value = self.read_ram(final_addr);
        self.x = new_value;

        self.update_nz(new_value);
        self.cycles += 3;
    }
    // 0xF9
    fn op_mov2_y_indexed_direct_page(&mut self) {
        let offset = self.read_ram(self.pc);
        self.pc += 1;

        let base_addr: u16 = if self.flag_set(PSWFlags::DirectPage) {
            0x0100
        } else {
            0x0000
        };
        let effective_offset = offset.wrapping_add(self.y);
        let final_addr = base_addr + (effective_offset as u16);
        let new_value = self.read_ram(final_addr);
        self.x = new_value;

        self.update_nz(new_value);
        self.cycles += 4;
    }

}
