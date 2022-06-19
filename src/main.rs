use std::fs::File;
use std::fs::OpenOptions;
use std::panic;
use std::process::Command;
extern crate job_scheduler;
use chrono::Local;
use job_scheduler::{Job, JobScheduler};
use std::path::Path;
use std::time::Duration;

fn get_temperature() -> f32 {
    let output = Command::new("sudo")
        .args(&["vcgencmd", "measure_temp"])
        .output()
        .expect("failed to get temperature");
    let temp = String::from_utf8_lossy(&output.stdout).into_owned();
    let temp = temp
        .strip_prefix("temp=")
        .expect("failed to strip prefix `temp=`")
        .strip_suffix("'C\n")
        .expect("failed to strip suffix ``C\\n`");
    let temp: f32 = temp.parse().expect("failed to parse str to f32");
    return temp;
}

fn save_temperature() {
    let dir_path = Path::new("./log");
    if dir_path.is_file() {
        panic!("dir_name");
    }
    let now_local = Local::now();
    let file_path = dir_path.join(now_local.format("%Y%m%d.csv").to_string());
    let f: File;
    f = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("failed to open a file");
    let mut wtr = csv::Writer::from_writer(f);
    wtr.serialize((
        now_local.format("%Y%m%d%H%M%S").to_string(),
        get_temperature(),
    ))
    .expect("failed to write data to csv");
    wtr.flush().expect("failed to flush data");
}

fn add_sched(sche_fn: fn() -> ()) {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("0/10 * * * * *".parse().unwrap(), || sche_fn()));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    add_sched(save_temperature);
}
