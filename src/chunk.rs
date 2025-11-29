/*
* each instruction has one byte operation code(opcode).
* it defines what kind of operations we're dealing with.
*/
pub enum OpCode {
    OPRETURN,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::OPRETURN => 0,
        }
    }
}

// FIXME: i think we dont need to add count and capacity since rust uses
// vec which is dynamically allocated.
/*
* chunk structure that holds bytecode and other data
*  - code: dynamic array of bytes representing bytecode
*/
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    /*
     * initializes a dynamically allocated chunk structure
     */
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    /*
     * writes a byte to chunk's array
     *  simply adds byte to the array
     */
    pub fn write_chunk<T>(&mut self, byte: T) 
    where T : Into<u8>{
        self.code.push(byte.into());
    }
}
