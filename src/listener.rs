use std::io::{Write, BufWriter, BufReader, BufRead};
use std::net::{TcpListener};
use crossbeam_channel::{Sender};
use crate::command::Command;
use colored::*;

pub trait CommandListener<'a> {
    fn buff_io(&self) -> (Box<dyn BufRead>, Box<dyn Write + Send + Sync>);
    fn sender(&self) -> &Sender<Command>;
    fn format_success(response: &str) -> String;
    fn format_error(response: &str) -> Option<String>;

    fn listen(&self) {
        loop {
            let mut line = String::new();
            let mut io = self.buff_io();

            io.0.read_line(&mut line).unwrap();

            self.sender().send(Command::from(
                line,
                move |result| match result {
                    Result::Ok(success) => {
                        io.1.write_all(Self::format_success(&success).as_bytes()).unwrap();
                        io.1.flush().unwrap();
                    }
                    Result::Err(fail) => {
                        if let Some(formatted) = Self::format_error(&fail) {
                            io.1.write_all(formatted.as_bytes()).unwrap();
                            io.1.flush().unwrap();
                        }
                    }
                }
            )).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
} 

pub struct TcpCommandListener<'a> {
    sender: &'a Sender<Command>,
    listen_port: u16
}

impl<'a> TcpCommandListener<'a> {
    pub fn new(sender: &Sender<Command>, listen_port: u16) -> TcpCommandListener {
        println!("{}", format!("Listening on port {}...", listen_port).blue());
        TcpCommandListener {
            sender: sender,
            listen_port: listen_port
        }
    }
}

impl<'a> CommandListener<'a> for TcpCommandListener<'a> {
    fn buff_io(&self) -> (Box<dyn BufRead>, Box<dyn Write + Send + Sync>) {
        let listener = TcpListener::bind(format!("localhost:{}", self.listen_port)).unwrap();
        let stream = listener.accept().expect("Cannot listen on network").0;

        (
            Box::new(BufReader::new(stream.try_clone().unwrap())), 
            Box::new(BufWriter::new(stream.try_clone().unwrap()))
        )
    }
    fn sender(&self) -> &Sender<Command> {
        &self.sender
    }
    fn format_success(response: &str) -> String {
        format!("{}\n", response.replace("\n", " "))
    }
    fn format_error(_response: &str) -> Option<String> {
        None
    }
}

pub struct CliCommandListener<'a> {
    sender: &'a Sender<Command>
}

impl<'a> CliCommandListener<'a> {
    pub fn new(sender: &Sender<Command>) -> CliCommandListener {
        CliCommandListener {
            sender: sender
        }
    }
}

impl<'a> CommandListener<'a> for CliCommandListener<'a> {
    fn buff_io(&self) -> (Box<dyn BufRead>, Box<dyn Write + Send + Sync>) {
        (
            Box::new(BufReader::new(std::io::stdin())), 
            Box::new(BufWriter::new(std::io::stdout()))
        )
    }
    fn sender(&self) -> &Sender<Command> {
        &self.sender
    }
    fn format_success(response: &str) -> String {
        format!("{} {}\n", "✓".green(), response.green())
    }
    fn format_error(response: &str) -> Option<String> {
        Some(format!("{} {}\n", "✕".red(), response.red()))
    }
}