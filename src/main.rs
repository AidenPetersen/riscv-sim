mod components;
mod clock;
mod instructions;
use crate::components::memory;
use actix::{clock::sleep, prelude::*};
use clap::Parser;
use components::{cache::L2Cache, memory::{Memory, QueueMemory}};
use clock::Clock;
use log;
use std::{path::PathBuf, time::Duration};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "ELF_FILE", help = "ELF file to run")]
    binary: PathBuf,

    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "stats.txt",
        help = "Output location of run stats"
    )]
    stats_file: PathBuf,

    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "trace.txt",
        help = "Output location of program trace"
    )]
    trace_file: PathBuf,
}

#[actix::main]
async fn main() {
    log::debug!("test");
    env_logger::init();
    let cli = Cli::parse();
    let nums: [u8; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ];    let dcache = L2Cache{}.start();
    let icache = L2Cache{}.start();
    let memory = QueueMemory::init(cli.binary, icache, dcache).start();
    let _ = memory.do_send(memory::ReadMessage{addr:0, memtype: memory::MemType::DMem, id: 0 });
    let _ = memory.do_send(memory::WriteMessage{addr:0x100000,memtype:memory::MemType::DMem,id:1, bytes: nums });
    let _ = memory.do_send(memory::ReadMessage{addr:0, memtype: memory::MemType::DMem, id: 4 });

    for i in 1..55 {
        println!("{}", i);
        let _ = memory.send(Clock).await;
    }
    let _ = memory.do_send(memory::ReadMessage{addr:0x100000, memtype: memory::MemType::DMem, id: 2 });
    for _ in 1..55 {
        let _ = memory.send(Clock).await;
    }

    sleep(Duration::from_secs(1)).await;
    println!("hi")

}
