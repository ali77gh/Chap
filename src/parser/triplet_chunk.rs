
use crate::common::executable::ExecutableLine;
use crate::common::chunk::Chunk;
use super::chunk_detector::chunk_detector as cd;
use crate::common::errors::Result;

fn double_chunk_parser(ch1: String, ch2: String, ch3: String) -> Result<ExecutableLine>{

    // match (cd(ch1), cd(ch2),cd(ch3)) {
    //     (Chunk::Params(_), Chunk::Params(_), Chunk::Params(_)) => todo!(),
    //     (Chunk::Params(_), Chunk::Params(_), Chunk::Function { name }) => todo!(),
    //     (Chunk::Params(_), Chunk::Function { name }, Chunk::Params(_)) => todo!(),
    //     (Chunk::Params(_), Chunk::Function { name }, Chunk::Function { name }) => todo!(),
    //     (Chunk::Function { name }, Chunk::Params(_), Chunk::Params(_)) => todo!(),
    //     (Chunk::Function { name }, Chunk::Params(_), Chunk::Function { name }) => todo!(),
    //     (Chunk::Function { name }, Chunk::Function { name }, Chunk::Params(_)) => todo!(),
    //     (Chunk::Function { name }, Chunk::Function { name }, Chunk::Function { name }) => todo!(),
    // }
    todo!()
}