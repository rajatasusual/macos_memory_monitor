use rustyline::{completion::Completer, Context};

pub struct ProcessCompleter {
    pub(crate) processes: Vec<(i32, String, u64)>, // (PID, Process Name, Memory Usage)
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
        let line_lower = line.to_lowercase(); // Convert input to lowercase for case-insensitive matching

        for (pid, name, _memory) in &self.processes {
            if name.to_lowercase().starts_with(&line_lower) 
                || pid.to_string().starts_with(line) 
            {
                completions.push(format!("{} - {}", pid, name));
            }
        }
        Ok((0, completions))
    }
}

pub struct ByteSize(pub u64);

impl ByteSize {
    pub fn format(&self) -> String {
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

pub struct TimeFormat(pub u64);

impl TimeFormat {
    pub fn format(&self) -> String {
        let seconds = self.0 / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        if days > 0 {
            format!("{}d", days)
        } else if hours > 0 {
            format!("{}h", hours)
        } else if minutes > 0 {
            format!("{}m", minutes)
        } else {
            format!("{}s", seconds)
        }
    }
}