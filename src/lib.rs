mod block;
mod chain;
mod network;
mod command;
mod listener;

use crate::chain::Chain;
use crate::network::Network;
use crossbeam_channel::{Receiver, Sender, unbounded};
use command::Command;
use listener::{TcpCommandListener, CliCommandListener, CommandListener};

pub fn run(listen_port: u16) {
    let mut chain = Chain::init();
    let mut network = Network::init();

    let (tcp_sender, cli_sender, receiver) = create_channel();
    let cli_listener = CliCommandListener::new(&cli_sender);
    let tcp_listener = TcpCommandListener::new(&tcp_sender, listen_port);

    crossbeam::scope(|scope| {
        scope.spawn(|| tcp_listener.listen());
        scope.spawn(|| cli_listener.listen());

        loop {
            match receiver.try_recv() {
                Ok(mut command) => {
                    let result = execute_command(command.command_line.clone(), &mut chain, &mut network);
                    command.handle_result(result);
                }
                Err(_) => {}
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn execute_command(command: String, chain: &mut Chain, network: &mut Network) -> Result<String, String> {
    let parts = command.split(' ').collect::<Vec<&str>>();
    match parts[0] {
        "addgen" => chain.add_genesis(),
        "addcontent" => chain.add_content(parts[1].to_owned()),
        "addpeer" => network.add_peer(parts[1].to_owned()),
        "sync" => network.get_main_branch(chain),
        "print" => Ok(chain.print()),
        &_ => Err(String::from("Possible commands: addgen, addcontent, addpeer, sync, print"))
    }
}

fn create_channel() -> (Sender<Command>, Sender<Command>, Receiver<Command>) {
    let (s, r) = unbounded();
    return (s.clone(), s, r);
}