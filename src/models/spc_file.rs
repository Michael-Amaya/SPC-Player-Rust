use std::collections::HashMap;
use num_traits::FromPrimitive;

use crate::models::enums::data_types::DataType;
use crate::models::enums::extended_id666_types::ExtendedID666Types;

const VERIFICATION_BYTE: u8 = 26;
const MINIMUM_PRINTABLE_CHAR:u8 = 0x20;
const MAXIMUM_PRINTABLE_CHAR:u8 = 0x7E;

#[allow(unused)]
#[derive(Debug)]
pub struct ExtendedID666Value {
    pub data_type: DataType,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SPCFile {
    pub header: [u8; 33],                            // 0x00000
    pub twenty_six_twenty_six: [u8; 2],              // 0x00021
    pub id666_info:  u8,                             // 0x00023
    pub version_minor: u8,                           // 0x00024
    pub pc: [u8; 2],                                 // 0x00025
    pub a: u8,                                       // 0x00027
    pub x: u8,                                       // 0x00028
    pub y: u8,                                       // 0x00029
    pub psw: u8,                                     // 0x0002A 
    pub sp: u8,                                      // 0x0002B
    pub reserved: [u8; 2],                           // 000002C
    pub text_song_title: [u8; 32],                   // 0x0002E
    pub text_game_title: [u8; 32],                   // 0x0004E
    pub text_dumper: [u8; 16],                       // 0x0006E
    pub text_comments: [u8; 32],                     // 0x0007E
    pub text_date_dumped: [u8; 11],                  // 0x0009E
    pub text_num_seconds_before_fading: [u8; 3],     // 0x000A9
    pub text_length_fade_ms: [u8; 5],                // 0x000AC
    pub text_artist: [u8; 32],                       // 0x000B1
    pub text_default_channel_disable: u8,            // 0x000D1
    pub text_emulator_used: u8,                      // 0x000D2
    pub text_reserved: [u8; 45],                     // 0x000D3
    pub binary_song_title: [u8; 32],                 // 0x0002E
    pub binary_game_title: [u8; 32],                 // 0x0004E
    pub binary_dumper: [u8; 16],                     // 0x0006E
    pub binary_comments: [u8; 32],                   // 0x0007E
    pub binary_date_dumped: [u8; 4],                 // 0x0009E
    pub binary_unused: [u8; 7],                      // 0x000A2
    pub binary_num_seconds_before_fading: [u8; 3],   // 0x000A9
    pub binary_length_fade_ms: [u8; 3],              // 0x000AC
    pub binary_artist: [u8; 32],                     // 0x000B0
    pub binary_default_channel_disable: u8,          // 0x000D0
    pub binary_emulator_used: u8,                    // 0x000D1
    pub binary_reserved: [u8; 46],                   // 0x000D2
    pub ram_64kb: [u8; 65536],                       // 0x00100
    pub dsp_registers: [u8; 128],                    // 0x10100
    pub unused: [u8; 64],                            // 0x10180
    pub extra_ram: [u8; 64],                         // 0x101C0

    pub contains_id666_info: bool,
    pub using_text: bool,
    pub extended_id666_info: HashMap<ExtendedID666Types, ExtendedID666Value>,
}

impl Default for SPCFile {
    fn default() -> Self {
        Self {
            header: [0; 33],
            twenty_six_twenty_six: [0; 2],
            id666_info: 0,
            version_minor: 0,
            pc: [0; 2],
            a: 0,
            x: 0,
            y: 0,
            psw: 0,
            sp: 0,
            reserved: [0; 2],
            text_song_title: [0; 32],
            text_game_title: [0; 32],
            text_dumper: [0; 16],
            text_comments: [0; 32],
            text_date_dumped: [0; 11],
            text_num_seconds_before_fading: [0; 3],
            text_length_fade_ms: [0; 5],
            text_artist: [0; 32],
            text_default_channel_disable: 0,
            text_emulator_used: 0,
            text_reserved: [0; 45],
            binary_song_title: [0; 32],
            binary_game_title: [0; 32],
            binary_dumper: [0; 16],
            binary_comments: [0; 32],
            binary_date_dumped: [0; 4],
            binary_unused: [0; 7],
            binary_num_seconds_before_fading: [0; 3],
            binary_length_fade_ms: [0; 3],
            binary_artist: [0; 32],
            binary_default_channel_disable: 0,
            binary_emulator_used: 0,
            binary_reserved: [0; 46],
            ram_64kb: [0; 65536],
            dsp_registers: [0; 128],
            unused: [0; 64],
            extra_ram: [0; 64],
            contains_id666_info: false,
            using_text: false,
            extended_id666_info: HashMap::new(),
        }
    }
}

impl SPCFile {
    pub fn new(data: Vec<u8>) -> Result<Self, String> {
        let minimum_size = 46;
        if data.len() < minimum_size {
            return Err("File is not a valid SPC file. Bad length".into());
        }

        let mut encoded_spc_file = SPCFile {..Default::default()};
        encoded_spc_file.header.copy_from_slice(data[0x00..0x0+33].try_into().unwrap());
        encoded_spc_file.twenty_six_twenty_six.copy_from_slice(data[0x21..0x21+2].try_into().unwrap());
        if encoded_spc_file.header[0..8] != *"SNES-SPC".as_bytes() || (encoded_spc_file.twenty_six_twenty_six[0] != VERIFICATION_BYTE && encoded_spc_file.twenty_six_twenty_six[1] != VERIFICATION_BYTE) {
            return Err("File is not a valid SPC file. Did not pass verification".into());
        }
        
        // println!("Header: {}, 2626: {:?}", std::str::from_utf8(&encoded_spc_file.header).unwrap(), encoded_spc_file.twenty_six_twenty_six);
        encoded_spc_file.id666_info = data[0x23];
        encoded_spc_file.version_minor = data[0x24];
        encoded_spc_file.pc.copy_from_slice(data[0x25..0x25+2].try_into().unwrap());
        encoded_spc_file.a = data[0x27];
        encoded_spc_file.x = data[0x28];
        encoded_spc_file.y = data[0x29];
        encoded_spc_file.psw = data[0x2A];
        encoded_spc_file.sp = data[0x2B];
        encoded_spc_file.reserved.copy_from_slice(data[0x2C..0x2C+2].try_into().unwrap());

        if encoded_spc_file.id666_info == VERIFICATION_BYTE {
            encoded_spc_file.contains_id666_info = true;
            encoded_spc_file.text_song_title.copy_from_slice(data[0x2E..0x2E+32].try_into().unwrap());
            encoded_spc_file.text_game_title.copy_from_slice(data[0x4E..0x4E+32].try_into().unwrap());
            encoded_spc_file.text_dumper.copy_from_slice(data[0x6E..0x6E+16].try_into().unwrap());
            encoded_spc_file.text_comments.copy_from_slice(data[0x7E..0x7E+32].try_into().unwrap());
            encoded_spc_file.text_date_dumped.copy_from_slice(data[0x9E..0x9E+11].try_into().unwrap());
            encoded_spc_file.text_num_seconds_before_fading.copy_from_slice(data[0xA9..0xA9+3].try_into().unwrap());
            encoded_spc_file.text_length_fade_ms.copy_from_slice(data[0xAC..0xAC+5].try_into().unwrap());
            encoded_spc_file.text_artist.copy_from_slice(data[0xB1..0xB1+32].try_into().unwrap());
            encoded_spc_file.text_default_channel_disable = data[0xD1];
            encoded_spc_file.text_emulator_used = data[0xD2];
            encoded_spc_file.text_reserved.copy_from_slice(data[0xD3..0xD3+45].try_into().unwrap());
        }

        if encoded_spc_file.contains_id666_info && check_id666_info_is_text(&encoded_spc_file) {
            encoded_spc_file.using_text = true;
        } else {
            encoded_spc_file.binary_song_title.copy_from_slice(data[0x2E..0x2E+32].try_into().unwrap());
            encoded_spc_file.binary_game_title.copy_from_slice(data[0x4E..0x4E+32].try_into().unwrap());
            encoded_spc_file.binary_dumper.copy_from_slice(data[0x6E..0x6E+16].try_into().unwrap());
            encoded_spc_file.binary_comments.copy_from_slice(data[0x7E..0x7E+32].try_into().unwrap());
            encoded_spc_file.binary_date_dumped.copy_from_slice(data[0x9E..0x9E+4].try_into().unwrap());
            encoded_spc_file.binary_unused.copy_from_slice(data[0xA2..0xA2+7].try_into().unwrap());
            encoded_spc_file.binary_num_seconds_before_fading.copy_from_slice(data[0xA9..0xA9+3].try_into().unwrap());
            encoded_spc_file.binary_length_fade_ms.copy_from_slice(data[0xAC..0xAC+3].try_into().unwrap());
            encoded_spc_file.binary_artist.copy_from_slice(data[0xB0..0xB0+32].try_into().unwrap());
            encoded_spc_file.binary_default_channel_disable = data[0xD0];
            encoded_spc_file.binary_emulator_used = data[0xD1];
            encoded_spc_file.binary_reserved.copy_from_slice(data[0xD2..0xD2+46].try_into().unwrap());
        }

        encoded_spc_file.ram_64kb.copy_from_slice(data[0x100..0x100+65536].try_into().unwrap());
        encoded_spc_file.dsp_registers.copy_from_slice(data[0x10100..0x10100+128].try_into().unwrap());
        encoded_spc_file.unused.copy_from_slice(data[0x10180..0x10180+64].try_into().unwrap());
        encoded_spc_file.extra_ram.copy_from_slice(data[0x101C0..0x101C0+64].try_into().unwrap());

        if data.len() > 0x10200+4 && str::from_utf8(&data[0x10200..0x10200+4]).unwrap() == "xid6" {
            let chunk_size = u32::from_le_bytes(data[0x10200+4..0x10200+8].try_into().unwrap());
            println!("Chunk size: {}", chunk_size);
            let mut remaining_size = chunk_size - 8; // already parsed 8 bytes from header
            let mut current_offset = 0x10208;
            while remaining_size > 0 {
                let id_byte = data[current_offset];
                let type_byte = data[current_offset+1];
                let type_type = DataType::from_u8(type_byte).unwrap();
                let data_bytes: Vec<u8> = data[current_offset+2..current_offset+4].try_into().unwrap();
                let id_type = ExtendedID666Types::from_u8(id_byte).unwrap();
                remaining_size -= 4;
                current_offset += 4;
                if type_byte == 0x0 {
                    // Data bytes contains the data
                    encoded_spc_file.extended_id666_info.insert(id_type, ExtendedID666Value {data_type: type_type, data: data_bytes});
                } else {
                    let data_length = u16::from_le_bytes(data_bytes.try_into().unwrap());
                    let start_location = current_offset;
                    let end_location = current_offset + data_length as usize;

                    if end_location > data.len() {
                        panic!("Extended ID666 data is too long?? start: {}, end: {}", start_location, end_location);
                    }

                    encoded_spc_file.extended_id666_info.insert(id_type, ExtendedID666Value { data_type: type_type, data: data[start_location..end_location].try_into().unwrap() });
                    current_offset += data_length as usize;
                    remaining_size -= data_length as u32;
                }
            }
        }
        
        Ok(encoded_spc_file)
    }
}

fn check_id666_info_is_text(spc_file: &SPCFile) -> bool {
    let to_check = vec![&spc_file.text_song_title[..], &spc_file.text_game_title[..], &spc_file.text_dumper[..], &spc_file.text_comments[..], &spc_file.text_date_dumped[..], &spc_file.text_num_seconds_before_fading[..], &spc_file.text_length_fade_ms[..], &spc_file.text_artist[..]];
    let to_pass = to_check.len() / 2;
    let mut num_failed = 0;
    for item in to_check {
        for char in item {
            if char < &MINIMUM_PRINTABLE_CHAR || char > &MAXIMUM_PRINTABLE_CHAR {
                num_failed += 1;
                break;
            }
        }
    }

    num_failed < to_pass
}