use crate::block::Block;
use json::object;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {
    pub fn init() -> Chain {
        Chain {
            blocks: Vec::new()
        }
    }

    pub fn add_content(&mut self, content: String) -> Result<String, String> {
        let prev_hash = self.last().get_hash().to_owned();
        let (block, result) = Block::mine(&prev_hash, content).unwrap();
        
        self.blocks.push(block);
        Ok(result)
    }
    
    pub fn add_genesis(&mut self) -> Result<String, String> {
        self.blocks.push(Block::genesis());
        Ok("Added genesis block".to_owned())
    }

    pub fn override_blocks(&mut self, other: &Chain) {
        self.blocks = other.blocks.clone();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.blocks.is_empty() {
            return Ok(());
        }

        let mut block_iter = self.blocks.iter();
        let mut prev_hash = block_iter.next().unwrap().get_hash(); // do not validate genesis block

        for block in block_iter {
            if let Err(error) = block.validate(prev_hash) {
                return Err(error);
            }
            prev_hash = block.get_hash();
        }

        Ok(())
    }
    
    pub fn print(&self) -> String {
        json::stringify_pretty(self, 2)
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    fn last(&self) -> &Block {
        &self.blocks.last().expect("Chain is not initialised")
    }
}

impl std::convert::From<&Chain> for json::JsonValue {
    fn from(chain: &Chain) -> Self {
        let mut block_refs = Vec::new();
        for block in chain.blocks.iter() {
            block_refs.push(block);
        }

        object!{
            blocks: block_refs
        }
    }
}

impl std::convert::From<json::JsonValue> for Chain {
    fn from(data: json::JsonValue) -> Self {
        let mut blocks = Vec::new();
        for block in data["blocks"].members() {
            blocks.push(Block::from(block));
        }

        return Chain {
            blocks: blocks
        }
    }
}

mod tests {
    use crate::chain::Chain;
    
    #[test]
    fn it_can_be_deserialized_from_serialized_form() {
        let mut valid_chain = Chain::init();
        valid_chain.add_genesis();
        valid_chain.add_content(String::from("hello-world"));

        let serialized_form = valid_chain.print();

        assert_eq!(Chain::from(json::parse(&serialized_form).unwrap()), valid_chain);
    }
}