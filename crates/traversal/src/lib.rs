use ethers::{prelude::{Http, Provider}, providers::Middleware, types::{Block, Transaction, H256}};
use eyre::Error;

pub struct Reader {
    node_client: Provider<Http>,
    traverse: Traversal,
}


pub struct Traversal {
    curr_block: Block<H256>,
    last_block: Option<Block<H256>>,
}


impl Traversal {
    pub async fn new(start_block: Block<H256>) -> Self {
        Traversal { curr_block: start_block, last_block: None }
    }
}

impl Reader {
    pub async fn new(node_client: Provider<Http>, start_block: Block<H256>) -> Self {
        Reader { node_client, traverse: Traversal::new(start_block).await }
    }

    pub async fn next(& mut self) -> Result<Block<H256>, Error> {
        let next_block = self.traverse.curr_block.number.unwrap().as_u64() + 1;
        // todo - add retry
        let next_block = self.node_client.get_block(next_block).await?;

        if next_block.is_none() {
            return Err(eyre::eyre!("Next block not found"));
        }

        self.traverse.last_block = Some(self.traverse.curr_block.clone());

        let curr_block = next_block.unwrap();

        self.traverse.curr_block = curr_block.clone();
        Ok(curr_block)
    }
}

