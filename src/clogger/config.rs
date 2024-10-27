use clap::Parser;

/// A scanning docker container-log monitoring tool.
///
/// Scans in given interval for new containers matching the given search query and starts displaying their logs.
/// If keywords were provided, they will be highlighted in the log output.
#[derive(Parser, Debug)]
#[command(version, about, author, long_about)]
pub struct Config {
    /// Display a tiling-line if the log pauses for more than 2 seconds
    #[arg(short, long, default_value_t = false)]
    pub time_tile: bool,
    /// Keywords to highlight in the log output, seperated by commas
    #[arg(short, long, value_delimiter = ',', required = false)]
    pub keywords: Vec<String>,
    // /// Partial Docker image or contaoner name to monitor
    // #[arg(short, long, required = true)]
    // pub search: String,
    /// Interval to scan for new containers in human readable format
    #[clap(short='i', long, value_parser = humantime::parse_duration, default_value = "500ms")]
    pub scan_interval: std::time::Duration,
    /// Path to the Docker socket
    #[arg(short, long, default_value = "unix:///var/run/docker.sock")]
    pub docker_socket: String,
    /// Command line to start child process
    #[clap(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    pub search_term: Vec<String>,
}

pub fn new() -> Config {
    Config::parse()
}
