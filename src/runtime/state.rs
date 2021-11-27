use crate::chunk::Chunk;
use crate::runtime::stack::LStack;
use std::sync::Arc;
use crate::chunk::opcode::FatIns;
use crate::chunk::proto::ProtoType;

pub struct LState {
    stack: LStack,
    proto: Arc<ProtoType>,
    pc: usize,
}

impl LState {
    pub fn new(stack_size: usize, proto: Arc<ProtoType>) -> Self {
        Self {
            stack: LStack::new(stack_size),
            proto,
            pc: 0,
        }
    }

    pub fn stack(&mut self) -> &mut LStack {
        &mut self.stack
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn fetch(&mut self) -> FatIns {
        let f = self.proto.code[self.pc];
        self.pc += 1;
        f
    }
}