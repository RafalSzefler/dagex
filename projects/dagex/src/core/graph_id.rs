use crate::GlobalId;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct GraphId {
    global_id: GlobalId
}

impl GraphId {    
    #[inline(always)]
    pub fn generate_next() -> Self {
        Self { global_id: GlobalId::generate_next() }
    }
}

impl From<GraphId> for i32 {
    #[inline(always)]
    fn from(value: GraphId) -> Self {
        i32::from(value.global_id)
    }
}
