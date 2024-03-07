struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2, // Example difficulty
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        // calculate hash here
        self.chain.push(genesis_block);
    }

    // Add methods for adding blocks, validating the chain, etc.
}
