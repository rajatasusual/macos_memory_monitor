extern crate libc;
extern crate libproc;
extern crate rustyline;
extern crate strsim;

mod process;
mod util;

use libproc::libproc::proc_pid;
use libproc::libproc::proc_pid::{listpids, ProcType};
use libproc::libproc::task_info::TaskAllInfo;
use process::KinfoProc;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Editor;
use rustyline::Helper;
use std::io;
use std::time::Duration;
use strsim::jaro_winkler;
use util::{ByteSize, ProcessCompleter};

impl Helper for ProcessCompleter {}
impl Hinter for ProcessCompleter {
    type Hint = String;
}
impl Highlighter for ProcessCompleter {}
impl Validator for ProcessCompleter {}

fn get_process_info(pid: i32) -> io::Result<()> {
    // Get the process name
    let process_name = match proc_pid::name(pid) {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Error: Unable to retrieve the process name for PID {}", pid);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Process name retrieval failed",
            ));
        }
    };

    // Get the memory usage information for the PID
    match proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
        Ok(info) => {
            let memory_usage = ByteSize(info.ptinfo.pti_resident_size);

            // Get CPU times using sysctl
            let mut mib: [i32; 4] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_PID, pid];
            let mut proc_info: KinfoProc = unsafe { std::mem::zeroed() };
            let mut proc_info_size = std::mem::size_of::<KinfoProc>() as usize;

            let result = unsafe {
                libc::sysctl(
                    mib.as_mut_ptr(),
                    mib.len() as u32,
                    &mut proc_info as *mut _ as *mut libc::c_void,
                    &mut proc_info_size,
                    std::ptr::null_mut(),
                    0,
                )
            };

            if result == -1 {
                eprintln!(
                    "Error: Unable to retrieve CPU times for PID {} using sysctl",
                    pid
                );
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "CPU times retrieval failed",
                ));
            }

            // Calculate CPU usage percentage
            let total_time = proc_info.kp_proc.p_uticks + proc_info.kp_proc.p_sticks;
            let elapsed_time = info.pbsd.pbi_start_tvsec as u64; // Assuming pbi_start_tvsec is in seconds since boot
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::new(0, 0))
                .as_secs();
            let process_uptime = current_time - elapsed_time;
            let cpu_usage = if process_uptime > 0 {
                ((total_time as f64 / process_uptime as f64) * 100.0) as u64
            } else {
                0
            };

            println!("PID: {}", pid);
            println!("Name: {}", process_name);
            println!("Memory Usage: {}", memory_usage.format());
            println!("CPU Usage: {}%", cpu_usage);
        }
        Err(_) => {
            eprintln!("Error: Unable to retrieve the process info for PID {}", pid);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Process info retrieval failed",
            ));
        }
    }

    Ok(())
}

fn get_best_matches<'a>(
    input: &str,
    processes: &'a Vec<(i32, String, u64)>,
) -> Vec<(f64, i32, String, u64)> {
    let mut best_matches = processes
        .iter()
        .map(|(pid, name, memory)| {
            let name_score = jaro_winkler(&input, name);
            let pid_score = if input.parse::<i32>().ok() == Some(*pid) {
                1.0
            } else {
                0.0
            }; // Exact PID match gets a perfect score
            let score = name_score.max(pid_score);
            (score, *pid, name.clone(), *memory)
        })
        .filter(|(score, _, _, _)| *score > 0.7)
        .collect::<Vec<_>>();

    best_matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    return best_matches;
}

fn main() -> io::Result<()> {
    // Get the list of all PIDs and process names
    let pids = listpids(ProcType::ProcAllPIDS)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut processes = Vec::new();

    for pid in pids {
        if pid == 0 {
            continue;
        }

        if let Ok(name) = proc_pid::name(pid as i32) {
            if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid as i32, 0) {
                let memory_usage = info.ptinfo.pti_resident_size;

                processes.push((pid as i32, name, memory_usage));
            }
        }
    }

    // Create the autocomplete engine with the process list
    let completer = ProcessCompleter {
        processes: processes.clone(),
    }; // Clone the processes vector
    let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    rl.set_helper(Some(completer));

    // Main loop to take user input with autocomplete
    loop {
        let readline = rl.readline("Enter PID or process name: ");
        match readline {
            Ok(input) => {
                let _ = rl.add_history_entry(input.as_str());

                let best_matches = get_best_matches(&input, &processes);

                if let Some((_, pid, _, _)) = best_matches.first() {
                    get_process_info(*pid)?;
                } else {
                    eprintln!("Error: No matching process found for input '{}'", input);
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
