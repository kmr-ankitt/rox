use crate::chunk::Chunk;

/*
* it disassembles the entire chunk by iterating through each instruction
*/
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset : usize = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!(" {}", name);
    offset + 1
}

/*
* offset shows where the instruction is located in the chunk
* disassembles a single instruction at given offset
* and returns the offset of the next instruction
*/
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    print!("{:04}", offset);

    let instruction = chunk.code[offset];
    match instruction {
        OPRETURN => simple_instruction("OPRETURN", offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}
