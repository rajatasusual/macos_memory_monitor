mod util;
mod matching;

use libproc::libproc::proc_pid;
use libproc::libproc::proc_pid::{listpids, ProcType};
use libproc::libproc::task_info::TaskAllInfo;
use matching::{get_best_matches, sort_by_memory, sort_by_cpu_time};
use prettytable::{row, Table};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Editor;
use rustyline::Helper;
use std::io;
use util::{ByteSize, ProcessCompleter, TimeFormat};

impl Helper for ProcessCompleter {}
impl Hinter for ProcessCompleter {
    type Hint = String;
}
impl Highlighter for ProcessCompleter {}
impl Validator for ProcessCompleter {}

fn get_process_info(pid: i32) -> io::Result<()> {
    let process_name = proc_pid::name(pid).map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to retrieve the process name for PID",
        )
    })?;

    let info = proc_pid::pidinfo::<TaskAllInfo>(pid, 0).map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to retrieve the process info for PID",
        )
    })?;

    let memory_usage = ByteSize(info.ptinfo.pti_resident_size);
    let total_time = TimeFormat(info.ptinfo.pti_total_user);

    let mut table = Table::new();
    
    table.add_row(row![bFg => "Attribute", "Value"]);

    // Add rows to the table

    table.add_row(row![bFb -> "Process ID", Fb-> &pid.to_string()]);
    table.add_row(row![bFb -> "Process Name", Fb-> &process_name]);
    table.add_row(row![bFb -> "Memory Usage", Fb-> &memory_usage.format()]);
    table.add_row(row![bFb -> "Total CPU Time", Fb-> &total_time.format()]);
    // Print the table
    table.printstd();

    Ok(())
}

fn main() -> io::Result<()> {
    let pids = listpids(ProcType::ProcAllPIDS)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let processes: Vec<(i32, String, u64)> = pids
        .into_iter()
        .filter(|&pid| pid != 0)
        .filter_map(|pid| {
            proc_pid::name(pid as i32).ok().and_then(|name| {
                proc_pid::pidinfo::<TaskAllInfo>(pid as i32, 0)
                    .ok()
                    .map(|info| (pid as i32, name, info.ptinfo.pti_resident_size))
            })
        })
        .collect();

    let completer = ProcessCompleter {
        processes: processes.clone(),
    };
    let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    rl.set_helper(Some(completer));

    loop {
        let readline = rl.readline("Enter PID or process name (filter: 'name:filter_text' or 'pid:filter_pid', sort: 'memory' or 'cpu'): ");
        match readline {
            Ok(input) => {
                let _ = rl.add_history_entry(input.as_str());

                let mut best_matches = get_best_matches(&input, &processes);

                // Apply sorting if requested
                if input.contains("sort:memory") {
                    sort_by_memory(&mut best_matches);
                } else if input.contains("sort:cpu") {
                    sort_by_cpu_time(&mut best_matches);
                }

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
