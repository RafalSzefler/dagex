#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Node {
    id: i32,
}

impl Node {
    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id
    }
}

impl From<i32> for Node {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}
