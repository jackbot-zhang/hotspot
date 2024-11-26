use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::default()))
        .build("/tmp/hotspot.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("up!!!");
    let mut binding = Command::new("/usr/bin/nmcli");
    let cmd = binding.arg("c").arg("show");
    loop {
        let output = cmd.output().unwrap();
        let text = String::from_utf8(output.stdout).unwrap();
        let rows: Vec<&str> = text.trim().split("\n").collect();
        for row in rows {
            let iter = row.split_whitespace();
            let items: Vec<&str> = iter.collect();
            if items[0] == "xxxxxx" && items[3] == "--" {
                up(items[1])
            }
        }

        sleep(Duration::from_secs(1))
    }
}


fn up(uuid: &str) {
    let mut binding = Command::new("/usr/bin/nmcli");
    let cmd = binding.arg("connection").arg("up").arg(uuid);
    let output = cmd.output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                log::error!("{:?}", o)
            }
        }
        Err(e) => {
            log::error!("{}", e.to_string())
        }
    }
}
