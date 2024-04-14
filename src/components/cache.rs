#[derive(Debug)]
pub enum CacheTransaction {
    Busy,
    ReadStarted,
    ReadDone(u32),
    WriteStarted,
    WriteDone,
}

/// Trait for Caches to implement reade and write operations.
pub trait Cache {
    /// Loads a byte from cache
    fn read_b(&mut self, addr: u32) -> &Transaction;
    /// Loads a half-word from storage
    fn read_h(&mut self, addr: u32) -> &Transaction;
    /// Loads a word from cache
    fn read_w(&mut self, addr: u32) -> &Transaction;

    /// Stores a byte to cache
    fn write_b(&mut self, addr: u32, val: u32) -> &Transaction;
    /// Stores a half-word to cache
    fn write_h(&mut self, addr: u32, val: u32) -> &Transaction;
    /// Stores a word to cache
    fn write_w(&mut self, addr: u32, val: u32) -> &Transaction;
    /// Flush  line
    fn flush_line(&mut self, addr: u32) -> &Transaction;
    /// Flush whole cache
    fn flush(&mut self) -> &Transaction;

}

// fn read_byte(&mut self, addr: u32) -> u32 {
//     if addr < self.elf_mem.len() as u32 {
//         self.elf_mem[addr as usize] as u32
//     } else {
//         *self.stack.entry(addr).or_default() as u32
//     }
// }
// fn write_byte(&mut self, addr: u32, val: u32) {
//     if addr < self.elf_mem.len() as u32 {
//         self.elf_mem[addr as usize] = val as u8;
//     } else {
//         self.stack.insert(addr, val as u8);
//     }
// }

// fn load_b(&mut self, addr: u32) -> u32 {
//     log::debug!("Memory: Loading byte from 0x{:#08x}", addr);
//     self.read_byte(addr)
// }
// fn load_h(&mut self, addr: u32) -> u32 {
//     log::debug!("Memory: Loading half-word from 0x{:#08x}", addr);
//     self.read_byte(addr) | (self.read_byte(addr) << 8)
// }
// fn load_w(&mut self, addr: u32) -> u32 {
//     log::debug!("Memory: Loading word from 0x{:#08x}", addr);
//     let result = self.read_byte(addr)
//         | (self.read_byte(addr + 1) << 8)
//         | (self.read_byte(addr + 2) << 16)
//         | (self.read_byte(addr + 3) << 24);
//     log::debug!("Memory: Loading word from 0x{:#08x} 0x{:#08x}", addr,  self.read_byte(addr));
//     result

// }

// fn store_b(&mut self, addr: u32, val: u32) {
//     log::debug!("Memory: Writing byte 0x{:02x} to 0x{:08x}", addr, val);
//     self.write_byte(addr, val)
// }
// fn store_h(&mut self, addr: u32, val: u32) {
//     log::debug!("Memory: Writing half-word 0x{:04x} to 0x{:08x}", addr, val);
//     let b0: u32 = val & 0xFF;
//     let b1: u32 = (val & 0xFF00) >> 8;
//     self.write_byte(addr, b1);
//     self.write_byte(addr + 1, b0);
// }
// fn store_w(&mut self, addr: u32, val: u32) {
//     log::debug!("Memory: Writing word 0x{:08x} to 0x{:08x}", addr, val);
//     let b0: u32 = val & 0xFF;
//     let b1: u32 = (val & 0xFF00) >> 8;
//     let b2: u32 = (val & 0xFF0000) >> 16;
//     let b3: u32 = (val & 0xFF000000) >> 24;
//     self.write_byte(addr, b3);
//     self.write_byte(addr + 1, b2);
//     self.write_byte(addr + 2, b1);
//     self.write_byte(addr + 3, b0);
// }