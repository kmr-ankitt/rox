/*
* each instruction has one byte operation code(opcode).
* it defines what kind of operations we're dealing with.
*/
pub enum OpCode{
    OPRETURN,
}

/*
* chunk structure that holds bytecode and other data
*  - capacity: total allocated size for code array
*  - count: current number of bytes used in code array
*  - code: dynamic array of bytes representing bytecode
*/
pub struct Chunk{
    pub count: usize,
    pub capacity: usize,
    pub code: Vec<u8>
}

impl Chunk{
    pub fn new() -> Self{
        Chunk { 
            count: 0, 
            capacity: 0, 
            code: Vec::new(),
        }
    }
}
