#![allow(clippy::new_without_default)]

use std::marker::PhantomData;

#[derive(Debug)]
pub struct ReadResult {
    read_bytes: usize,
}

impl ReadResult {
    pub fn new(read_bytes: usize) -> Self {
        Self { read_bytes }
    }

    pub fn read_bytes(&self) -> usize { self.read_bytes }
}

#[derive(Debug)]
pub struct WriteResult {
    _phantom: PhantomData<()>,
}

impl WriteResult {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

#[derive(Debug)]
pub struct FlushResult {
    _phantom: PhantomData<()>,
}

impl FlushResult {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}
