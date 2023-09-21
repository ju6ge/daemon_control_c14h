use std::{os::unix::net::{UnixStream, UnixListener}, path::Path};
use std::io::Write;
use std::io::Read;

use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Parser)]
enum Command {
    Pid,
    Ping
}

impl Command {
   fn handle(&self) {
       match self {
        Command::Pid => {
           println!("do this")
        },
        Command::Ping => {
           println!("do that")
        },
        }
   }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Option<Command>
}


fn main() {
    let cmd_options = Options::parse();

    const SOCKET_PATH: &str = "/tmp/socket.sock";

    match cmd_options.command {
        Some(command) => {
            let _ = UnixStream::connect(SOCKET_PATH).and_then(|mut connection| {
                writeln!(connection, "{}", serde_json::to_string(&command).unwrap());
            Ok(())
            });
        },
        None => {
        UnixListener::bind(SOCKET_PATH).and_then(|listener| {
            for connection in listener.incoming() {
                let _ = connection.and_then(|mut stream| {
                    let mut buf = String::new();
                    let _ = stream.read_to_string(&mut buf);
                    let command: Command = serde_json::from_str(&buf).unwrap();
                    command.handle();
                    Ok(())
                });
            }
            Ok(())
            });
        },
    }
}
