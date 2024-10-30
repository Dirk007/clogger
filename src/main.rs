use anyhow::Result;
use colored::Colorize;

mod clogger;
use clogger::{
    logprinter::{consume_logs, print_now},
    FindQuery, LogFilter,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = clogger::config::new();

    let search = config.search_term.join(" ");
    println!("clogger scanning for '*{}*'...", search);

    let query = FindQuery::new(search);
    let log = LogFilter::new(config.keywords.as_ref());

    let docker = docker_api::Docker::new(&config.docker_socket)?;
    let containers = docker_api::Containers::new(docker);
    let list_opts = docker_api::opts::ContainerListOpts::builder().all(true).build();
    loop {
        let c = containers.list(&list_opts).await?;
        for item in c {
            if query.is_match(&item) {
                print_now();
                println!(
                    "Found container {} from {}",
                    clogger::helper::extract_name(&item).red().on_blue(),
                    clogger::helper::extract_image(&item).red().on_green()
                );
                if let Some(id) = item.id {
                    let log_source = containers.get(id);
                    consume_logs(log_source, config.time_tile, &log).await.ok();
                    println!("{}", "<container stopped>".red().on_blue());
                    break;
                }
            }
        }

        tokio::time::sleep(config.scan_interval).await;
    }
}
