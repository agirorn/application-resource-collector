use sysinfo::{ProcessExt, ProcessRefreshKind, System, SystemExt};
use human_bytes::human_bytes;
use regex::Regex;
use crate::sample;

pub fn new_system(process_name_pattern: &Regex) -> System {
    let mut system = System::new_all();
    system.refresh_processes_specifics(ProcessRefreshKind::everything());
    for (_pid, process) in system.processes() {
        if process_name_pattern.is_match(process.name()) {
            process.cpu_usage();
        }
    }
    return system;
}

pub fn collect_process_sample(
    system: &mut System,
    process_name_pattern: &Regex,
) -> sample::ProcessSample {
    system.refresh_processes_specifics(ProcessRefreshKind::everything());
    let processes: Vec<sample::Process> = system
        .processes()
        .iter()
        .filter(|(_pid, process)| process_name_pattern.is_match(process.name()))
        .map(|(pid, process)| {
            let memory = process.memory();
            sample::Process {
                pid: pid.to_string(),
                status: process.status().to_string(),
                memory,
                human_bytes: human_bytes(memory as f64),
                cpu_usage: process.cpu_usage(),
                name: process.name().to_owned(),
                cmd: process.cmd().join(" "),
            }
        })
        .collect();
    sample::ProcessSample::with_processes(processes)
}
