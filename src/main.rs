use clap::Parser;
use regex::Regex;
use std::process;
use std::{thread, time::Duration};
use sysinfo::{Pid, PidExt};

mod collect;
mod sample;
use crate::collect::*;
use crate::sample::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Save collected samples to file"
    )]
    out: Option<String>,

    #[arg(
        short,
        long,
        value_name = "DELAY",
        default_value = "1000",
        help = "The delay between samples in millisecones"
    )]
    delay: Option<u32>,
}

fn main() -> std::io::Result<()> {
    let pid = Pid::from_u32(process::id());
    let mut process_name: String = ".*slow.*".to_owned();
    let mut delay_ms: u32 = 0;
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        process_name = name.to_string();
    }
    if let Some(delay) = cli.delay {
        delay_ms = delay
    }
    let delay = Duration::from_millis(delay_ms.into());
    let process_name_pattern = Regex::new(&format!("{}", &process_name).to_owned()).unwrap();
    let mut system = new_system(pid, &process_name_pattern);
    loop {
        thread::sleep(delay);
        let sample = collect_process_sample(pid, &mut system, &process_name_pattern);
        if let Some(out) = cli.out.as_deref() {
            append_to_file(out, &sample);
        }
        print_table(sample);

        thread::sleep(delay);
    }
}
