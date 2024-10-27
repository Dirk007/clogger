use colored::Colorize;

pub struct LogFilter {
    keywords: Vec<String>,
    replacements: Vec<String>,
}

impl LogFilter {
    pub fn new(keywords: &[impl ToString]) -> Self {
        let mut log_filter = LogFilter {
            replacements: Vec::with_capacity(keywords.len()),
            keywords: Vec::with_capacity(keywords.len()),
        };

        for w in keywords {
            let w = w.to_string();
            log_filter.replacements.push(w.red().on_yellow().bold().to_string());
            log_filter.keywords.push(w);
        }

        log_filter
    }

    pub fn highlight(&self, mut line: String) -> String {
        for (idx, w) in self.keywords.iter().enumerate() {
            let replacement = &self.replacements[idx];
            line = line.replace(w.as_str(), replacement);
        }

        line
    }
}
