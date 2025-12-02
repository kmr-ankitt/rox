mod chunk;
mod debug;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.write_chunk(chunk::OpCode::OPRETURN);
    debug::disassemble_chunk(&chunk, "test chunk");
}
