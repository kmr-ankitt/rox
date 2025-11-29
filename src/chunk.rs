/*
* each instruction has one byte operation code(opcode).
* it defines what kind of operations we're dealing with.
*/
pub enum OpCode{
    OPRETURN,
}

// FIXME: i think we dont need to add count and capacity since rust uses 
// vec which is dynamically allocated.
/*
* chunk structure that holds bytecode and other data
*  - code: dynamic array of bytes representing bytecode
*/
pub struct Chunk{
    pub code: Vec<u8>
}

impl Chunk{

    /*
     * initializes a dynamically allocated chunk structure
     */
    pub fn new() -> Self{
        Chunk { 
            code: Vec::new(),
        }
    }

    /*
    * writes a byte to chunk's array
    *  simply adds byte to the array
    */
    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
