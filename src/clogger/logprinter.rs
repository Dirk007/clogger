use anyhow::Result;
use colored::Colorize;
use futures_util::{io::AsyncBufReadExt, stream::StreamExt};

use crate::LogFilter;

pub fn print_now() {
    let now = chrono::Local::now();
    print!("[{}] ", now.format("%Y-%m-%d %H:%M:%S").to_string().bright_green());
}
pub async fn consume_logs(c: docker_api::Container, time_tile: bool, filter: &LogFilter) -> Result<()> {
    let log_opts = docker_api::opts::LogsOptsBuilder::default()
        .follow(true)
        .stdout(true)
        .stderr(true)
        .build();
    let mut log_stream = c.logs(&log_opts);

    let mut got_some = false;
    'printloop: loop {
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                if got_some && time_tile {
                    print_now();
                    println!("{}", "------------------ <log pause> ------------------".green())
                }
                got_some = false;
            }
            chunk = log_stream.next() => {
                got_some = true;
                print_now();
                if let Some(Ok(chunk)) = chunk {
                    let lines = chunk.lines();
                    let mut l = lines.map(|l| l.unwrap_or_default());
                    while let Some(line) = l.next().await {
                        println!("{}", filter.highlight(line));
                    }
                } else {
                    break 'printloop
                }
            }
        }
    }

    Ok(())
}
