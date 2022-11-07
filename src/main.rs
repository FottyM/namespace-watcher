extern crate cronjob;
use cronjob::CronJob;
use std::fs;
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;

fn watch(_name: &str) {
    let mut kubectl = Command::new("kubectl");

    let cmd = kubectl
        .arg("get")
        .arg("deployments.apps")
        .arg("--output=jsonpath='{.items[*].metadata.name}'")
        .stdin(Stdio::piped())
        .output()
        .expect("Unable to get service list");

    let output_string = String::from_utf8(cmd.stdout).expect("Unable get string output");

    let vec = output_string.replace("'", "");
    let vec: Vec<&str> = vec.split(" ").collect();

    for service in vec {
        let logs = get_service_logs(service).unwrap();

        if logs.contains("Fortunat") {
            println!("service: {}", service)
        }

        fs::write(String::from(service) + ".log", logs).unwrap();
    }
}

fn get_service_logs(name: &str) -> Result<String, FromUtf8Error> {
    let app = String::from("deployments.apps/") + name;

    let output = Command::new("kubectl")
        .arg("logs")
        .arg(app)
        .output()
        .expect("Could not get service logs")
        .stdout;

    String::from_utf8(output)
}

fn main() {
    let mut cron = CronJob::new("Namespace watcher", watch);
    cron.seconds("*/30");
    cron.minutes("*");
    cron.day_of_week("*");
    cron.offset(2);
    cron.start_job();
}
