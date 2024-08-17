extern crate libc;
extern crate libproc;
extern crate rustyline;
extern crate strsim;

use libproc::libproc::proc_pid;
use libproc::libproc::proc_pid::{listpids, ProcType};
use libproc::libproc::task_info::TaskAllInfo;
use rustyline::Editor;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use strsim::jaro_winkler;
use std::io;

struct ProcessCompleter {
    processes: Vec<(i32, String, u64)>, // (PID, Process Name, Memory Usage)
}
struct ByteSize(u64);

impl ByteSize {
    fn format(&self) -> String {
        let sizes = ["B", "KB", "MB", "GB", "TB"];
        let mut size = self.0 as f64;
        let mut i = 0;
        while size >= 1024.0 && i < sizes.len() - 1 {
            size /= 1024.0;
            i += 1;
        }
        format!("{:.2} {}", size, sizes[i])
    }
}

impl Completer for ProcessCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut completions = Vec::new();
        for (pid, name, _memory) in &self.processes {
            if name.starts_with(line) || pid.to_string().starts_with(line) {
                completions.push(format!("{} - {}", pid, name));
            }
        }
        Ok((0, completions))
    }
}

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
            return Err(io::Error::new(io::ErrorKind::Other, "Process name retrieval failed"));
        }
    };

    // Get the memory usage information for the PID
    match proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
        Ok(info) => {
            let memory_usage = ByteSize(info.ptinfo.pti_resident_size);
            println!("PID: {}", pid);
            println!("Name: {}", process_name);
            println!("Memory Usage: {}", memory_usage.format());
        }
        Err(_) => {
            eprintln!("Error: Unable to retrieve the process info for PID {}", pid);
            return Err(io::Error::new(io::ErrorKind::Other, "Process info retrieval failed"));
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Get the list of all PIDs and process names
    let pids = listpids(ProcType::ProcAllPIDS).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
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
    let completer = ProcessCompleter { processes: processes.clone() }; // Clone the processes vector
    let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    rl.set_helper(Some(completer));

    // Main loop to take user input with autocomplete
    loop {
        let readline = rl.readline("Enter PID or process name: ");
        match readline {
            Ok(input) => {
                let _ = rl.add_history_entry(input.as_str());

                let mut best_matches = processes.iter()
                    .map(|(pid, name, memory)| {
                        let score = jaro_winkler(&input, name);
                        (score, *pid, name.clone(), *memory)
                    })
                    .filter(|(score, _, _, _)| *score > 0.7) // Only consider matches with a score above 0.7
                    .collect::<Vec<_>>();

                best_matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

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