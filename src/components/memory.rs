use actix::prelude::*;
use log;
use std::{collections::HashMap, path::PathBuf};

use crate::clock::Clock;

use super::cache::L2Cache;
const DMEM_TRANSACTIONS: u32 = 2;
const DMEM_TRANSACTIONS_U: usize = DMEM_TRANSACTIONS as usize;

const IMEM_TRANSACTIONS: u32 = 2;
const IMEM_TRANSACTIONS_U: usize = IMEM_TRANSACTIONS as usize;

const ACCESS_CYCLES: u32 = 50;
const BLOCK_SIZE: u32 = 1 << 6;
const BLOCK_SIZE_U: usize = BLOCK_SIZE as usize;

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum MemType {
    IMem,
    DMem,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct WriteMessage {
    pub addr: u32,
    pub bytes: [u8; BLOCK_SIZE_U],
    pub memtype: MemType,
    pub id: u32,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum WriteRespMessage {
    Busy { id: u32 },
    Success { id: u32 },
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct ReadMessage {
    pub addr: u32,
    pub memtype: MemType,
    pub id: u32,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum ReadRespMessage {
    Success { bytes: [u8; BLOCK_SIZE_U], id: u32 },
    Busy,
}

/// All of the required types for a memory module
pub trait Memory: Handler<ReadMessage> + Handler<WriteMessage> + Handler<Clock> + Actor {
    fn init(elf_file: PathBuf, icache: Addr<L2Cache>, dcache: Addr<L2Cache>) -> Self;
}

/// Queue Entry for QueueMemory
#[derive(Debug, Clone)]
enum MemoryQueueEntry {
    Write(WriteMessage, u32),
    Read(ReadMessage, u32),
}

/// Memory implementation that queues a limited amount of instructions
#[derive(Debug, Clone)]
pub struct QueueMemory {
    imem_queue: Vec<MemoryQueueEntry>,
    dmem_queue: Vec<MemoryQueueEntry>,
    icache: Addr<L2Cache>,
    dcache: Addr<L2Cache>,

    elf_mem: Vec<u8>,
    stack: HashMap<u32, u8>,
}

impl Memory for QueueMemory {
    fn init(elf_file: PathBuf, icache: Addr<L2Cache>, dcache: Addr<L2Cache>) -> Self {
        let file_data = std::fs::read(elf_file).expect("Could not read file.");
        let slice = file_data.as_slice();

        Self {
            imem_queue: Vec::new(),
            dmem_queue: Vec::new(),
            icache,
            dcache,

            elf_mem: file_data,
            stack: HashMap::new(),
        }
    }
}

impl QueueMemory {
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

impl Actor for QueueMemory {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(usize::max_value())
    }
}

impl Handler<ReadMessage> for QueueMemory {
    type Result = ();
    fn handle(&mut self, msg: ReadMessage, ctx: &mut Self::Context) {
        let msg_ref = &msg;
        match msg.memtype {
            MemType::IMem => {
                if self.imem_queue.len() >= IMEM_TRANSACTIONS_U {
                    self.icache.do_send(ReadRespMessage::Busy);
                } else {
                    log::debug!(
                        "ID {}: Read to 0x{:08x} in IMem queued in Memory",
                        msg.id,
                        &msg.addr
                    );
                    self.imem_queue.push(MemoryQueueEntry::Read(msg, 0));
                }
            }
            MemType::DMem => {
                if self.dmem_queue.len() >= IMEM_TRANSACTIONS_U {
                    self.dcache.do_send(ReadRespMessage::Busy);
                } else {
                    log::debug!(
                        "ID {}: Read to 0x{:08x} in DMem queued in Memory",
                        msg_ref.id,
                        msg_ref.addr
                    );
                    self.dmem_queue.push(MemoryQueueEntry::Read(msg, 0));
                }
            }
        }
    }
}

impl Handler<WriteMessage> for QueueMemory {
    type Result = ();
    fn handle(&mut self, msg: WriteMessage, ctx: &mut Self::Context) {
        let msg_ref = &msg;
        match msg.memtype {
            MemType::IMem => {
                if self.imem_queue.len() >= IMEM_TRANSACTIONS_U {
                    self.icache.do_send(ReadRespMessage::Busy);
                } else {
                    log::debug!(
                        "ID {}: Write to 0x{:08x} in IMem queued in Memory",
                        msg_ref.id,
                        msg_ref.addr
                    );
                    self.imem_queue.push(MemoryQueueEntry::Write(msg, 0));
                }
            }
            MemType::DMem => {
                if self.dmem_queue.len() >= IMEM_TRANSACTIONS_U {
                    self.dcache.do_send(ReadRespMessage::Busy);
                } else {
                    log::debug!(
                        "ID {}: Write to 0x{:08x} in DMem queued in Memory",
                        msg_ref.id,
                        msg_ref.addr
                    );
                    self.dmem_queue.push(MemoryQueueEntry::Write(msg, 0));
                }
            }
        }
    }
}

impl Handler<Clock> for QueueMemory {
    type Result = ();

    fn handle(&mut self, _: Clock, _: &mut Self::Context) {
        for i in self.imem_queue.iter().chain(self.dmem_queue.iter()) {
            match i {
                MemoryQueueEntry::Write(_, mut c) => c += 1,
                MemoryQueueEntry::Read(_, mut c) => c += 1,
            }
        }
        let mut i = 0;
        while i < self.imem_queue.len() {
            match &mut self.imem_queue[i] {
                MemoryQueueEntry::Write(m, c) => {
                    if *c >= ACCESS_CYCLES {
                        let addr = m.addr - m.addr % BLOCK_SIZE;
                        for (idx, j) in m.bytes.iter().enumerate() {
                            QueueMemory::write_byte(
                                &mut self.elf_mem,
                                &mut self.stack,
                                addr + idx as u32,
                                *j,
                            );
                        }
                        self.icache.do_send(WriteRespMessage::Success { id: m.id });
                        log::debug!("ID {}: Completed IMem Write", &m.id);

                        self.imem_queue.remove(i);
                    } else {
                        *c += 1;
                        i += 1;
                    }
                }
                MemoryQueueEntry::Read(m, c) => {
                    if *c >= ACCESS_CYCLES {
                        let addr = m.addr - m.addr % BLOCK_SIZE;
                        let mut bytes: [u8; BLOCK_SIZE_U] = [0; BLOCK_SIZE_U];
                        for j in 0..BLOCK_SIZE {
                            bytes[j as usize] = QueueMemory::read_byte(
                                &mut self.elf_mem,
                                &mut self.stack,
                                addr + j,
                            );
                        }
                        self.icache.do_send(ReadRespMessage::Success {
                            id: m.id,
                            bytes: bytes,
                        });
                        log::debug!("ID {}: Completed IMem Read", &m.id);
                        self.imem_queue.remove(i);
                    } else {
                        *c += 1;
                        i += 1;
                    }
                }
            }
        }
        while i < self.dmem_queue.len() {
            match &mut self.dmem_queue[i] {
                MemoryQueueEntry::Write(m, c) => {
                    if *c >= ACCESS_CYCLES {
                        let addr = m.addr - m.addr % BLOCK_SIZE;
                        for (idx, j) in m.bytes.iter().enumerate() {
                            QueueMemory::write_byte(
                                &mut self.elf_mem,
                                &mut self.stack,
                                addr + idx as u32,
                                *j,
                            );
                        }
                        self.dcache.do_send(WriteRespMessage::Success { id: m.id });
                        log::debug!("ID {}: Completed DMem Write", &m.id);

                        self.dmem_queue.remove(i);
                    } else {
                        *c += 1;
                        i += 1;
                    }
                }
                MemoryQueueEntry::Read(m, c) => {
                    if *c >= ACCESS_CYCLES {
                        let addr = m.addr - m.addr % BLOCK_SIZE;
                        let mut bytes: [u8; BLOCK_SIZE_U] = [0; BLOCK_SIZE_U];
                        for j in 0..BLOCK_SIZE {
                            bytes[j as usize] = QueueMemory::read_byte(
                                &mut self.elf_mem,
                                &mut self.stack,
                                addr + j,
                            );
                        }
                        self.dcache.do_send(ReadRespMessage::Success {
                            id: m.id,
                            bytes: bytes,
                        });
                        log::debug!("ID {}: Completed DMem Read", &m.id);
                        self.dmem_queue.remove(i);
                    } else {
                        *c += 1;
                        i += 1;
                    }
                }
            }
        }
    }
}
