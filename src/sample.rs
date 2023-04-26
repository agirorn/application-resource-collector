use chrono::Local;
use tabled::{
    settings::{object::{Rows, Columns}, Alignment, Modify, Style},
    Table, Tabled,
};
use std::borrow::Cow;
use serde::{Serialize};
use std::fs::{OpenOptions, File};
use std::io::Write;

use human_bytes::human_bytes;

// use serde::{Serialize, Serializer};
// use sysinfo::ProcessStatus;

#[derive(Debug, Tabled, Serialize)]
pub struct Process {
    #[tabled(rename = "PID")]
    pub pid: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Memory")]
    pub memory: u64,
    // Having the human_bytes here is probably a mistake
    #[tabled(rename = "Memory")]
    pub human_bytes: String,
    #[tabled(rename = "CPU usage")]
    pub cpu_usage: f32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "CMD")]
    pub cmd: String,
}

#[derive(Debug, Default, Serialize)]
pub struct ProcessSample {
    pub timestamp: i64,
    pub memory: u64,
    pub cpu_usage: f32,
    pub processes: Vec<Process>
}

impl ProcessSample {
    pub fn with_processes(processes: Vec<Process>) -> Self {
        let mut sample = ProcessSample::default();
        sample.timestamp = Local::now().timestamp();
        sample.memory = processes.iter().map(|p| p.memory).sum();
        sample.cpu_usage = processes.iter().map(|p| p.cpu_usage).sum();
        sample.processes = processes;
        sample
    }
}

impl Tabled for ProcessSample {
    const LENGTH: usize = 5;

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            "Timestamp".into(),
            "Memory".into(),
            "Memory".into(),
            "CPU usage".into(),
            "Processes".into(),
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let timestamp = self.timestamp.to_string().into();
        let cpu_usage = format!("{:0.2} %", self.cpu_usage).into();
        let memory = self.memory;
        let processes: String = match &self.processes.len() {
            0 => String::from(""),
            _ =>  Table::new(&self.processes)
                // .with(Style::rounded())
                .with(Style::psql())
                .with(Alignment::right())
                .with(Modify::new(Columns::single(6)).with(Alignment::left()))
                .with(Modify::new(Rows::single(0)).with(Alignment::left()))
                .to_string()
        };

        vec![
            timestamp,
            memory.to_string().into(),
            human_bytes(memory as f64).into(),
            cpu_usage,
            processes.into()
        ]
    }
}


pub fn print_table(sample: ProcessSample) {
        println!(
            "{}",
            Table::new(vec![sample])
            .with(Style::re_structured_text().remove_bottom())
            .with(Alignment::right())
            .with(Modify::new(Rows::single(0)).with(Alignment::left()))
        );

}

fn new_file(name: &str) -> File {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(name)
        .unwrap()
}

pub fn append_to_file(filename: &str, sample: &ProcessSample) {
    let mut file: File = new_file(filename);
    let json = serde_json::to_string(&sample).unwrap();
    writeln!(file, "{}", json).unwrap();
}
