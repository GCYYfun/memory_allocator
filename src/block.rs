#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub start:usize,
    pub size:usize,
}

impl Block {
    pub fn empty()->Block{
        Block {
            start:0,
            size:0,
        }
    }

    pub fn new(start:usize,size:usize) -> Block{
        Block {
            start:start,
            size:size,
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        return false;
    }

    pub fn renew(&mut self,start:usize,size:usize){
        self.start = start;
        self.size = size;
    }
}