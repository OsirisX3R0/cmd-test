use clap::{App, Arg, SubCommand};
use std::process::Command;

fn main() {
    let matches = App::new("cmnd")
        .version("1.0.0")
        .author("Richard Harris <richardharris916@gmail.com>")
        .about("An app to try out making a CLI")
        .subcommand(
            SubCommand::with_name("echo")
                .about("Echo text in the terminal")
                .version("1.0.0")
                .author("Richard Harris <richardharris916@gmail.com>")
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .help("Prints text")
                        .value_name("TEXT")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("net")
                .about("Runs netstat")
                .version("1.0.0")
                .author("Richard Harris <richardharris916@gmail.com>"), // .arg(Arg::with_name("n: &'a str"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("echo") {
        if matches.is_present("text") {
            let value = matches.value_of("text").unwrap_or("nothing");
            let message = format!("echo {}", value);
            let output = Command::new("cmd")
                .args(&["/C", &message])
                .output()
                .expect("failed to execute");

            println!("Running 'echo {}'.....", value);
            println!("{}", String::from_utf8(output.stdout).unwrap());
            println!("Exited successfully!")
        }
    }

    if let Some(matches) = matches.subcommand_matches("net") {
        let output = Command::new("cmd")
            .args(&["/C", "netstat"])
            .output()
            .expect("failed to execute");

        println!("Running 'netstat'.....");
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("Exited successfully!")
    }
}
