mod cli;
mod config;
mod metrics;

extern crate daemonize;
extern crate systemstat;

use chrono::{DateTime, Utc};
use cli::start_cli;
use daemonize::Daemonize;
use metrics::{mount, memory};
use std::fs::File;
use std::{thread, time};
use systemstat::{Platform, System};
use sysinfo::{System as SystemInfo};

use std::io::Write;

fn main() {
    let matches = start_cli();

    let user = matches.value_of("USER").unwrap_or("nobody");
    let group = matches.value_of("GROUP").unwrap_or("daemon");
    let file = matches.value_of("OUTPUT").unwrap_or("./daemon.txt");

    // Defining where stdout and stderr should be redirected
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    // Defining the interval in with we want to track the metrics
    let sleep_time = time::Duration::from_millis(3 * 1000);

    // Opening the output file
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file)
        .unwrap();

    // Creating the daemon
    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start` takes a `pid_file` argument.
        .chown_pid_file(true) // Change the owner of the pid file to the specified user.
        .user(user) // Change the user to the specified user.
        .group(group) // Change the group to the specified group.
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| println!("Privileged action!")); // Run a privileged action before daemonizing.);

    // Creating the system stat
    let sys = System::new();

    // Creating the system info
    let mut sys_info = SystemInfo::new_all();

    // First we update all information of our `System` struct.
    sys_info.refresh_all();

    match daemonize.start() {
        Ok(_) => loop {
            // Sample code printing the current time every interval.
            let now: DateTime<Utc> = Utc::now();
            let date = format!("\nUTC now is: {}\n", now);
            file.write_all(date.as_bytes()).unwrap();

            // Reading File System Metrics
            match mount::read_metric(&sys) {
                Ok(metric) => {
                    // Saving metric to file
                    file.write_all(format!("\nFilesystem:\n{}\n", metric).as_bytes())
                        .unwrap();
                }
                Err(err) => {
                    let now: DateTime<Utc> = Utc::now();
                    file.write_all(format!("{}: Mount error: {}\n", now, err).as_bytes())
                        .unwrap()
                }
            }

            // Reading Memory Metrics
            if let Ok(metric) = memory::read_metric(&sys_info) {
                // Saving metric to file
                file.write_all(format!("\nMemory:\n{}\n", metric).as_bytes()).unwrap();
            }

            // Sleep for the specified interval.
            thread::sleep(sleep_time);
        },
        Err(err) => {
            let now: DateTime<Utc> = Utc::now();
            file.write_all(format!("{}: error: {}\n", now, err).as_bytes())
                .unwrap()
        }
    };
}
