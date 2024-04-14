mod components;
mod instructions;

use clap::Parser;
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use log;

use crate::components::{component::Component, memory::{MemType, Memory, QueueMem}};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "ELF_FILE", help = "ELF file to run")]
    binary: PathBuf,

    #[arg(short, long, value_name = "FILE", default_value = "stats.txt", help = "Output location of run stats")]
    stats_file: PathBuf,

    #[arg(short, long, value_name = "FILE", default_value = "trace.txt", help = "Output location of program trace")]
    trace_file: PathBuf,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    log::info!("Loading elf into memory...");
    let (mut mem, pc) = QueueMem::load_elf(cli.binary);
    log::info!("Loaded elf into memory");
    let nums: [u8;64] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63];
    let t1 = mem.write_block(0x00000000, nums, MemType::DMem);
    mem.cycle();
    let t2: Rc<RefCell<components::memory::MemoryTransaction>> = mem.read_block(0x00000000, MemType::DMem);
    log::info!("{:?}", *t1);
    log::info!("{:?}", *t2);

    for _ in 0..60 {
        mem.cycle();
    }
    log::info!("{:?}", *t1);
    log::info!("{:?}", *t2);


}
