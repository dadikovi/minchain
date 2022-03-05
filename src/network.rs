use std::net::{TcpStream};
use crate::chain::Chain;
use std::io::{Write, BufReader, BufRead};

pub struct Network {
    peers: Vec<String>
}

impl Network {
    pub fn init() -> Network {
        Network {
            peers: Vec::new()
        }
    }
    
    pub fn add_peer(&mut self, address: String) -> Result<String, String> {
        let maybe_stream = TcpStream::connect(address.to_owned());

        if let Ok(_) = maybe_stream {
            self.peers.push(address);
            return Ok(format!("Added new peer"));
        }
        else {
            return Err(format!("Couldn't add peer with address {}", &address));
        }
    }

    pub fn get_main_branch(&self, chain: &mut Chain) -> Result<String, String> {
        let mut main_branch_candidate: Option<Chain> = None;
        let mut top_chain_size = chain.len();

        for peer_address in self.peers.iter() {
            let maybe_stream = TcpStream::connect(peer_address.to_owned());

            if let Ok(mut stream) = maybe_stream {
                let request = b"print\n";
                let mut response = String::new();

                stream.set_nodelay(true).unwrap();
                stream.write_all(request).unwrap();
                stream.flush().unwrap();
                BufReader::new(stream.try_clone().unwrap()).read_line(&mut response).unwrap();
    
                if let Ok(chain_json) = json::parse(&response) {
                    let chain = Chain::from(chain_json);
                    let current_chain_size = chain.len();
                    if chain.validate() == Ok(()) && current_chain_size > top_chain_size {
                        main_branch_candidate = Some(chain);
                        top_chain_size = current_chain_size
                    }
                }
            }
        }

        if let Some(main_branch) = main_branch_candidate {
            chain.override_blocks(&main_branch);
            Ok("Got new main branch from peers".to_owned())
        } else {
            Ok("Our branch is the main branch".to_owned())
        }
    }
}