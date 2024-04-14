use super::component::Component;
use elf::{endian::LittleEndian, section::SectionHeader, ElfBytes};
use log;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;

const DMEM_TRANSACTIONS: u32 = 2;
const DMEM_TRANSACTIONS_U: usize = DMEM_TRANSACTIONS as usize;

const IMEM_TRANSACTIONS: u32 = 2;
const IMEM_TRANSACTIONS_U: usize = IMEM_TRANSACTIONS as usize;

const ACCESS_CYCLES: u32 = 50;
const BLOCK_SIZE: u32 = 1 << 6;
const BLOCK_SIZE_U: usize = BLOCK_SIZE as usize;

#[derive(Debug, Clone, Copy)]
pub enum MemoryTransaction {
    Busy,
    ReadStarted,
    ReadDone([u8; BLOCK_SIZE_U]),
    WriteStarted([u8; BLOCK_SIZE_U]),
    WriteDone,
}

#[derive(Debug, Clone, Copy)]
pub enum MemType {
    IMem,
    DMem,
}

pub trait Memory {
    fn read_block(&mut self, addr: u32, mem_type: MemType) -> Rc<RefCell<MemoryTransaction>>;
    fn write_block(
        &mut self,
        addr: u32,
        val: [u8; BLOCK_SIZE_U],
        mem_type: MemType,
    ) -> Rc<RefCell<MemoryTransaction>>;
}

#[derive(Debug)]
struct QueueEntry {
    cycle_counter: u32,
    addr: u32,
    transaction: Rc<RefCell<MemoryTransaction>>,
}

#[derive(Debug)]
pub struct QueueMem {
    /// Memory copied in from ELF file
    elf_mem: Vec<u8>,

    /// Stack region of memory, starting at 0x40000000 and decreasing
    stack: HashMap<u32, u8>,

    /// Queue of transactions for imem
    imem_queue: Vec<QueueEntry>,

    /// Queue of transactions for dmem
    dmem_queue: Vec<QueueEntry>,
}
impl QueueMem {
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

        (
            Self {
                elf_mem: file_data,
                stack: HashMap::new(),
                imem_queue: Vec::new(),
                dmem_queue: Vec::new(),
            },
            text_header.sh_offset as u32,
        )
    }

    fn read_byte(elf_mem: &mut Vec<u8>, mut stack: &mut HashMap<u32, u8>, addr: u32) -> u8 {
        if addr < elf_mem.len() as u32 {
            elf_mem[addr as usize] as u8
        } else {
            *stack.entry(addr).or_default() as u8
        }
    }
    fn write_byte(elf_mem: &mut Vec<u8>, mut stack: &mut HashMap<u32, u8>, addr: u32, val: u8) {
        if addr < elf_mem.len() as u32 {
            elf_mem[addr as usize] = val as u8;
        } else {
            stack.insert(addr, val as u8);
        }
    }
}

impl Memory for QueueMem {
    fn read_block(&mut self, addr: u32, mem_type: MemType) -> Rc<RefCell<MemoryTransaction>> {
        let block_start = addr - addr % BLOCK_SIZE;
        match mem_type {
            MemType::IMem => {
                if self.imem_queue.len() < IMEM_TRANSACTIONS_U {
                    let transaction = Rc::new(RefCell::new(MemoryTransaction::ReadStarted));
                    self.imem_queue.push(QueueEntry {
                        cycle_counter: 0,
                        transaction: Rc::clone(&transaction),
                        addr: block_start,
                    });
                    log::debug!("Read from 0x{:08x} in IMem queued in Memory", addr);

                    Rc::clone(&transaction)
                } else {
                    log::debug!("Memory unit is busy, transaction will be ignored");

                    Rc::new(RefCell::new(MemoryTransaction::Busy))
                }
            }
            MemType::DMem => {
                if self.dmem_queue.len() < DMEM_TRANSACTIONS_U {
                    let transaction = Rc::new(RefCell::new(MemoryTransaction::ReadStarted));
                    self.dmem_queue.push(QueueEntry {
                        cycle_counter: 0,
                        transaction: Rc::clone(&transaction),
                        addr: block_start,
                    });
                    log::debug!("Read from 0x{:08x} in DMem queued in Memory", addr);

                    Rc::clone(&transaction)
                } else {
                    log::debug!("Memory unit is busy, transaction will be ignored");

                    Rc::new(RefCell::new(MemoryTransaction::Busy))
                }
            }
        }
    }

    fn write_block(
        &mut self,
        addr: u32,
        val: [u8; BLOCK_SIZE_U],
        mem_type: MemType,
    ) -> Rc<RefCell<MemoryTransaction>> {
        let block_start = addr - addr % BLOCK_SIZE;
        match mem_type {
            MemType::IMem => {
                if self.imem_queue.len() < IMEM_TRANSACTIONS_U {
                    let transaction = Rc::new(RefCell::new(MemoryTransaction::WriteStarted(val)));
                    self.imem_queue.push(QueueEntry {
                        cycle_counter: 0,
                        transaction: Rc::clone(&transaction),
                        addr: block_start,
                    });
                    log::debug!("Write to 0x{:08x} in IMem queued in Memory", addr);

                    Rc::clone(&transaction)
                } else {
                    log::debug!("Memory unit is busy, transaction will be ignored");

                    Rc::new(RefCell::new(MemoryTransaction::Busy))
                }
            }
            MemType::DMem => {
                if self.dmem_queue.len() < DMEM_TRANSACTIONS_U {
                    let transaction = Rc::new(RefCell::new(MemoryTransaction::WriteStarted(val)));
                    self.dmem_queue.push(QueueEntry {
                        cycle_counter: 0,
                        transaction: Rc::clone(&transaction),
                        addr: block_start,
                    });
                    log::debug!("Write to 0x{:08x} in DMem queued in Memory", addr);

                    Rc::clone(&transaction)
                } else {
                    log::debug!("Memory unit is busy, transaction will be ignored");

                    Rc::new(RefCell::new(MemoryTransaction::Busy))
                }
            }
        }
    }
}

impl Component for QueueMem {
    fn cycle(&mut self) {
        // Update transactions
        let mut i = 0;
        while i < self.dmem_queue.len() {
            if self.dmem_queue[i].cycle_counter >= ACCESS_CYCLES {
                let transaction = Rc::clone(&self.dmem_queue[i].transaction);
                let mt = *(*transaction).borrow_mut();
                match mt {
                    MemoryTransaction::ReadStarted => {
                        let qe: &mut Vec<QueueEntry> = &mut self.dmem_queue;

                        let mut result: [u8; BLOCK_SIZE_U] = [0;64];

                        for j in 0..BLOCK_SIZE {
                            result[j as usize] = QueueMem::read_byte(&mut self.elf_mem, &mut self.stack, qe[i].addr + j);
                        }
                        *(*transaction).borrow_mut() = MemoryTransaction::ReadDone(result);

                        log::debug!("Read from 0x{:08x} in DMem completed", qe[i].addr);
                        qe.remove(0);

                    }
                    MemoryTransaction::WriteStarted(data) => {
                        let qe: &mut Vec<QueueEntry> = &mut self.dmem_queue;

                        for j in 0..BLOCK_SIZE {
                            QueueMem::write_byte(&mut self.elf_mem, &mut self.stack, qe[i].addr + j, data[j as usize]);
                        }
                        *(*transaction).borrow_mut() = MemoryTransaction::WriteDone;
                        log::debug!("Write to 0x{:08x} in DMem completed", qe[i].addr);
                        qe.remove(0);

                    }
                    _ => unreachable!("MemoryTransaction should only be ReadStarted or Writestarted when in queues")
                };
            } else {
                let qe = &mut self.dmem_queue[i];
                qe.cycle_counter += 1;
                i += 1;
            }
        }
        i = 0;
        while i < self.imem_queue.len() {
            if self.imem_queue[i].cycle_counter >= ACCESS_CYCLES {
                let transaction = Rc::clone(&self.imem_queue[i].transaction);
                let mt = *(*transaction).borrow_mut();
                match mt {
                    MemoryTransaction::ReadStarted => {
                        let qe: &mut Vec<QueueEntry> = &mut self.imem_queue;

                        let mut result: [u8; BLOCK_SIZE_U] = [0;64];

                        for j in 0..BLOCK_SIZE {
                            result[j as usize] = QueueMem::read_byte(&mut self.elf_mem, &mut self.stack, qe[i].addr + j);
                        }
                        *(*transaction).borrow_mut() = MemoryTransaction::ReadDone(result);

                        log::debug!("Read from 0x{:08x} in IMem completed", qe[i].addr);
                        qe.remove(0);

                    }
                    MemoryTransaction::WriteStarted(data) => {
                        let qe: &mut Vec<QueueEntry> = &mut self.imem_queue;

                        for j in 0..BLOCK_SIZE {
                            QueueMem::write_byte(&mut self.elf_mem, &mut self.stack, qe[i].addr + j, data[j as usize]);
                        }
                        
                        *(*transaction).borrow_mut() = MemoryTransaction::WriteDone;
                        log::debug!("Write to 0x{:08x} in IMem completed", qe[i].addr);
                        qe.remove(0);

                    }
                    _ => unreachable!("MemoryTransaction should only be ReadStarted or Writestarted when in queues")
                };
            } else {
                let qe = &mut self.imem_queue[i];
                qe.cycle_counter += 1;
                i += 1;
            }
        }
    }
}
