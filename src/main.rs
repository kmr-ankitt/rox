mod chunk;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.write_chunk(chunk::OpCode::OPRETURN);
    println!("{:?}", chunk);
}
