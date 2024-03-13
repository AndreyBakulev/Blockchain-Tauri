
mod block;
pub struct Blockchain {
    //Vec<Block> is an array of blocks
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        //makes a whole new blockchain
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2, // Example difficulty
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        //the things passed into new are index, data, and prev hash
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        // calculate hash here
        self.chain.push(genesis_block);
    }

    // Add methods for adding blocks, validating the chain, etc.
}