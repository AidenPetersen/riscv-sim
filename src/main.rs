use clap::Parser;
use std::path::PathBuf;
use log;

use crate::storage::Storage;
mod memory;
mod storage;
mod instructions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "BIN_FILE", help = "Binary file to run (baremetal RISC-V)")]
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

    let res = memory::Memory::load_elf(cli.binary);
    let mut mem = res.0;
    let pc = res.1;
    log::info!("Loaded elf into memory");
    for n in 0..10 {
        let pc = pc + 4 * n;
        let data: u32 = mem.load_w(pc);
        println!("Decode: {:?}", instructions::decode_inst(data))
    }
}
