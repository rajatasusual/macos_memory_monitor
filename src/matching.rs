use std::time::Duration;

use libproc::libproc::{proc_pid, task_info::TaskAllInfo};
use strsim::jaro_winkler;

pub fn get_best_matches<'a>(
    input: &str,
    processes: &'a Vec<(i32, String, u64)>,
) -> Vec<(f64, i32, String, u64)> {
    // Split the input into search term and sorting request (if any)
    let parts: Vec<&str> = input.split("sort:").collect();
    let search_term = parts[0].trim();

    let input_lower = search_term.to_lowercase();

    processes
        .iter()
        .filter(|(pid, name, _)| {
            // Check for hyphenated format or individual PID/name matches
            let formatted_process = format!("{} - {}", pid, name).to_lowercase();
            formatted_process.contains(&input_lower) 
                || name.to_lowercase().contains(&input_lower) 
                || pid.to_string().contains(&input_lower) 
        })
        .map(|(pid, name, memory)| {
            let name_score = jaro_winkler(&input_lower, &name.to_lowercase());
            let pid_score = if search_term.parse::<i32>().ok() == Some(*pid) { 1.0 } else { 0.0 }; 
            let formatted_score = jaro_winkler(&input_lower, &format!("{} - {}", pid, name).to_lowercase());
            let score = name_score.max(pid_score).max(formatted_score); // Consider the highest score
            (score, *pid, name.clone(), *memory)
        })
        .filter(|(score, _, _, _)| *score > 0.7) 
        .collect()
}

pub fn sort_by_memory(matches: &mut Vec<(f64, i32, String, u64)>) {
    matches.sort_by(|a, b| b.3.cmp(&a.3)); // Sort by memory usage (descending)
}

pub fn sort_by_cpu_time(matches: &mut Vec<(f64, i32, String, u64)>) {
    // Assuming you have CPU time information available in the `processes` vector
    // You might need to adjust this based on how you're storing CPU time
    matches.sort_by(|a, b| {
        let a_cpu_time = get_cpu_time(a.1); // Replace with your actual CPU time retrieval logic
        let b_cpu_time = get_cpu_time(b.1);
        b_cpu_time.cmp(&a_cpu_time) // Sort by CPU time (descending)
    });
}

pub fn get_cpu_time(pid: i32) -> Duration {
    match proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
        Ok(all_info) => {
            let total_time_microseconds = all_info.ptinfo.pti_total_user;
            Duration::from_micros(total_time_microseconds as u64)
        }
        Err(_) => {
            eprintln!("Error: Unable to retrieve CPU times for PID {}", pid);
            Duration::from_secs(0) // Return 0 duration on error
        }
    }
}