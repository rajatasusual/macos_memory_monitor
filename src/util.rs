use rustyline::{completion::Completer, Context};

pub struct ProcessCompleter {
    pub(crate) processes: Vec<(i32, String, u64)>, // (PID, Process Name, Memory Usage)
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
