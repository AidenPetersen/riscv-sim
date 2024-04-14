use crate::storage::Storage;
use elf::{endian::LittleEndian, section::SectionHeader, ElfBytes};
use log;
use std::collections::HashMap;
use std::path::PathBuf;

const ELF_END: u32 = 0x10000000;

#[derive(Debug)]
pub struct Memory {
    /// Program Counter
    pub pc: u32,

    /// Memory copied in from ELF file
    pub elf_mem: Vec<u8>,

    /// Stack region of memory, starting at 0x40000000 and decreasing
    pub stack: HashMap<u32, u8>,
}
impl Memory {
    /// Construct a new Memory object by loading elf file into elf_mem and creating empry stack
    pub fn load_elf(elf_path: PathBuf) -> (Self, u32) {
        // Opening elf file
        let file_data = std::fs::read(elf_path).expect("Could not read file.");
        let slice = file_data.as_slice();
        let elf_file =
            ElfBytes::<LittleEndian>::minimal_parse(slice).expect("ELF file should be parsable");
        // Finding text section to get offset
        let text_header: SectionHeader = elf_file
            .section_header_by_name(".text")
            .expect("Section table should be parseable")
            .expect("file should have a .text section");

        (Self {
            pc: text_header.sh_offset as u32,
            elf_mem: file_data,
            stack: HashMap::new(),
        }, text_header.sh_offset as u32)
    }

    fn read_byte(&mut self, addr: u32) -> u32 {
        if addr < self.elf_mem.len() as u32 {
            self.elf_mem[addr as usize] as u32
        } else {
            *self.stack.entry(addr).or_default() as u32
        }
    }
    fn write_byte(&mut self, addr: u32, val: u32) {
        if addr < self.elf_mem.len() as u32 {
            self.elf_mem[addr as usize] = val as u8;
        } else {
            self.stack.insert(addr, val as u8);
        }
    }
}
impl Storage for Memory {
    fn load_b(&mut self, addr: u32) -> u32 {
        log::debug!("Memory: Loading byte from 0x{:#08x}", addr);
        self.read_byte(addr)
    }
    fn load_h(&mut self, addr: u32) -> u32 {
        log::debug!("Memory: Loading half-word from 0x{:#08x}", addr);
        self.read_byte(addr) | (self.read_byte(addr) << 8)
    }
    fn load_w(&mut self, addr: u32) -> u32 {
        log::debug!("Memory: Loading word from 0x{:#08x}", addr);
        let result = self.read_byte(addr)
            | (self.read_byte(addr + 1) << 8)
            | (self.read_byte(addr + 2) << 16)
            | (self.read_byte(addr + 3) << 24);
        log::debug!("Memory: Loading word from 0x{:#08x} 0x{:#08x}", addr,  self.read_byte(addr));
        result

    }

    fn store_b(&mut self, addr: u32, val: u32) {
        log::debug!("Memory: Writing byte 0x{:02x} to 0x{:08x}", addr, val);
        self.write_byte(addr, val)
    }
    fn store_h(&mut self, addr: u32, val: u32) {
        log::debug!("Memory: Writing half-word 0x{:04x} to 0x{:08x}", addr, val);
        let b0: u32 = val & 0xFF;
        let b1: u32 = (val & 0xFF00) >> 8;
        self.write_byte(addr, b1);
        self.write_byte(addr + 1, b0);
    }
    fn store_w(&mut self, addr: u32, val: u32) {
        log::debug!("Memory: Writing word 0x{:08x} to 0x{:08x}", addr, val);
        let b0: u32 = val & 0xFF;
        let b1: u32 = (val & 0xFF00) >> 8;
        let b2: u32 = (val & 0xFF0000) >> 16;
        let b3: u32 = (val & 0xFF000000) >> 24;
        self.write_byte(addr, b3);
        self.write_byte(addr + 1, b2);
        self.write_byte(addr + 2, b1);
        self.write_byte(addr + 3, b0);

    }
}
