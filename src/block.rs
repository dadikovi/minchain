use std::fmt;
use std::str;
use sha2::{Sha256, Digest};
use rand::Rng;
use std::time::{Instant};
use json::object;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Block {
    // header
    prev_hash: String,
    salt: u64,
    hash: String,

    //content
    content: String
}

impl Block {
    pub fn mine(prev_hash: &String, content: String) -> Result<(Block, String), String> {  
        let mut hash: String;
        let mut salt: u64;
        let start = Instant::now(); 

        // This is the part I hate about blockchain :D
        loop {
            salt = rand::thread_rng().gen();
            hash = gen_hash(prev_hash, salt, &content);

            if pow(&hash) {
                return Ok((Block {
                    prev_hash: prev_hash.to_owned(),
                    salt: salt,
                    hash: hash,
                    content: content
                }, format!("Mined a new block in: {:?}", start.elapsed())))
            }
        }
    }

    pub fn genesis() -> Block {
        Block {
            prev_hash: String::from("GENESIS"),
            salt: 0,
            hash: String::from("GENESIS"),
            content: String::from("GENESIS")
        }
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
    
    pub fn validate(&self, expected_prev_hash: &str) -> Result<(), String> {
        if *self == Block::genesis() {
            return Ok(());
        }
        if expected_prev_hash != self.prev_hash {
            return Err(format!("Block {} does not point to the latest block: {}", self.hash, expected_prev_hash));
        }
        if gen_hash(&self.prev_hash, self.salt, &self.content) != self.hash || !pow(&self.hash) {
            return Err(format!("Hash {} is not valid", self.hash));
        }
        
        Ok(())
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        Block {
            prev_hash: self.prev_hash.to_owned(),
            salt: self.salt,
            hash: self.hash.to_owned(),
            content: self.content.to_owned()
        }
    }
}

impl std::convert::From<&Block> for json::JsonValue {
    fn from(chain: &Block) -> Self {
        object!{
            prev_hash: chain.prev_hash.to_owned(),
            salt: chain.salt,
            hash: chain.hash.to_owned(),
            content: chain.content.to_owned()
        }
    }
}

impl std::convert::From<&json::JsonValue> for Block {
    fn from(data: &json::JsonValue) -> Self {
        Block {
            prev_hash: data["prev_hash"].as_str().unwrap().to_owned(),
            salt: data["salt"].as_u64().unwrap(),
            hash: data["hash"].as_str().unwrap().to_owned(),
            content: data["content"].as_str().unwrap().to_owned()
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(prev_hash: {}\n hash: {}:{}\n content: {})\n\n", self.prev_hash, self.salt, self.hash, self.content)
    }
}

fn pow(hash: &str) -> bool {
    str::ends_with(hash, "00")
}

fn gen_hash(prev_hash: &str, salt: u64, content: &str) -> String {
    let data_to_hash = format!("{}{:x}{}", prev_hash, salt, content);
    format!("{:x}",Sha256::digest(data_to_hash))
}