
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Node {
    id: i32,
}

impl Node {
    #[inline(always)]
    pub fn new(id: i32) -> Self {
        Self { id: id }
    }

    #[inline(always)]
    pub fn get_numeric_id(&self) -> i32 {
        self.id
    }
}
