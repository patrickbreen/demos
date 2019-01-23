
/// This represents one block of memory
#[derive(Clone, Debug)]
pub struct Block {

    /// The starting address of this block
    start: usize,

    /// The length of this block
    length: usize,

    /// If this block is readonly, then it is not writable (such as a ROM)
    readonly: bool,

    /// The memory is stored as a vector of bytes
    memory: Vec<u8>,
}

impl Block {
    /// The constructor allocates and initializes an empy block of the given specifications.
    /// If you want to use an existing block. IE for loading a ROM, then that can be optionally
    /// supplied, and it will be checked for proper sizing.
    pub fn new(start: usize, length: usize, readonly: bool, memory: Option<Vec<u8>>) -> Block {

        if length == 0 {
            panic!("Error, tried to initialize illegal memory block.");
        }
        // check memory is exists and/or is right size
        let new_memory: Vec<u8> = match memory {
            Some(mut existing_memory) => {
                let existing_len = existing_memory.len();
                existing_memory.append(&mut vec![0; (length - existing_len)]);
                existing_memory
            },
            None => {
                vec![0; length]
            }
        };

        // return block
        Block {
            start: start,
            length: length,
            readonly: readonly,
            memory: new_memory,
        }
    }
}

/// This represents a series of memory blocks, and exposes access to those blocks
///
/// Once the MMU is set up, either by creating it with a collection of blocks in the constructor
/// or by using the `add_block` method, the MMU is primarily interacted with using the `read`, 
/// `read_word`, `write`, `write_word` methods.
///
/// Examples of the proper (and improper) use of the MMU can be seen in the tests below.
///
/// Note that technically this is a 8bit data bus, 16bit address bus architecture. I am using
/// a lot of usize types to hold the addresses out of convenience, but to be accurate to
/// to the emulation, they should be u16 address types. The methods read and write could each
/// be completed in one memory cycle on the target machine.
#[derive(Debug)]
pub struct MMU {
    blocks: Vec<Block>,
}

impl MMU {
    pub fn new(blocks: &Vec<Block>) -> MMU {

        let mut mmu = MMU {
            blocks: Vec::new(),
        };

        for block in blocks {
            mmu.add_block(block);
        }
        mmu
    }

    fn reset(&mut self) {
        // set all values to zero in all writable blocks

        for mut block in &mut self.blocks {
            if !block.readonly {
                block.memory = vec![0; block.length];
            }
        }

    }

    pub fn add_block(&mut self, new_block: &Block) {
        // check if new block overlaps with existing blocks
        for block in &self.blocks {
            let new_end_intersects = 
                new_block.start+new_block.length > block.start &&
                new_block.start+new_block.length < block.start + block.length;

            let new_start_intersects = 
                block.start+block.length > new_block.start &&
                block.start+block.length < new_block.start + new_block.length;

            if new_end_intersects || new_start_intersects {
                panic!("Error, add memory overlap error.")
            }
        }
        
        if new_block.length == 0 || new_block.memory.len() != new_block.length {
            panic!("Error, tried to initialize illegal memory block.");
        }
        self.blocks.push(new_block.clone());

    }

    // note this returns a copy of the Block
    fn get_block(&mut self, addr: usize) -> &mut Block {
        let mut index = 0;
        for block in &mut self.blocks {
            if addr >= block.start && addr < block.start + block.length {
                return block;
            }
            index += 1;
        }
        panic!("Error, block not found.");
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        let block = self.get_block(addr);
        // let block = &mut self.blocks[block_number];

        // check if block is writable
        if block.readonly   {
            panic!("Error, attempted to write to readonly memory.")
        }
        let block_start = block.start;
        block.memory[addr - block_start] = value;
    }

    pub fn read(&mut self, addr: usize) -> u8 {
        let block = self.get_block(addr);
        // let block = &self.blocks[block_number];
        let index = addr - block.start;
        block.memory[index]
    }

    pub fn read_word(&mut self, addr: usize) -> u16 {
        ((self.read(addr+1) as u16) << 8) + (self.read(addr) as u16)
    }
}

#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test_create_empty() {
        let mmu = MMU::new(&Vec::new());
    }

    #[test]
    fn test_create() {
        let mut blocks = Vec::new();
        blocks.push(Block::new(0, 128, false, None));
        let mut mmu = MMU::new(&blocks);
        blocks.push(Block::new(128, 256, false, None));
        mmu = MMU::new(&blocks);
    }

    #[test]
    fn test_create_with_list() {
        let mut blocks = Vec::new();
        let mut memory = vec![0; 128];
        memory[0] = 1;
        memory[1] = 2;
        memory[2] = 3;
        blocks.push(Block::new(0, 128, false, Some(memory)));
        let mmu = MMU::new(&blocks);

        assert_eq!(mmu.blocks[0].memory[0], 1);
        assert_eq!(mmu.blocks[0].memory[1], 2);
        assert_eq!(mmu.blocks[0].memory[2], 3);
    }

    #[test]
    #[should_panic]
    fn test_create_overlapping() {
        let mut blocks = Vec::new();
        blocks.push(Block::new(0, 129, false, None));
        let mut mmu = MMU::new(&blocks);
        blocks.push(Block::new(128, 256, false, None));
        mmu = MMU::new(&blocks);
    }

    #[test]
    fn test_add_block() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));
        mmu.add_block(&Block::new(128, 128, false, None));
    }

    #[test]
    #[should_panic]
    fn test_add_block_overlapping() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 129, false, None));
        mmu.add_block(&Block::new(128, 256, false, None));
    }

    #[test]
    fn test_write() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));

        mmu.write(16, 25);
        assert_eq!(mmu.blocks[0].memory[16], 25);
    }

    #[test]
    fn test_write_multiple_blocks() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));
        mmu.add_block(&Block::new(128, 128, false, None));

        mmu.write(16, 25);
        mmu.write(130, 14);
        assert_eq!(mmu.blocks[0].memory[16], 25);
        assert_eq!(mmu.blocks[1].memory[2], 14);
    }

    #[test]
    #[should_panic]
    fn test_write_readonly() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, true, None));

        mmu.write(16, 25);
    }

    #[test]
    fn test_read() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));

        mmu.write(16, 25);
        assert_eq!(mmu.read(16), 25);
    }

    #[test]
    #[should_panic]
    fn test_index_error() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));

        mmu.write(128, 25);
    }

    // test_reset
    #[test]
    fn test_reset() {
        let mut mmu = MMU::new(&Vec::new());
        mmu.add_block(&Block::new(0, 128, false, None));

        mmu.write(16, 25);
        mmu.reset();
        assert_eq!(mmu.read(16), 0);
    }
}