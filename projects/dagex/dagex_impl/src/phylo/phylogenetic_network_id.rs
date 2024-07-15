use crate::GlobalId;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct PhylogeneticNetworkId {
    global_id: GlobalId
}

impl PhylogeneticNetworkId {    
    #[inline(always)]
    pub fn generate_next() -> Self {
        Self { global_id: GlobalId::generate_next() }
    }
}

impl From<PhylogeneticNetworkId> for i32 {
    #[inline(always)]
    fn from(value: PhylogeneticNetworkId) -> Self {
        i32::from(value.global_id)
    }
}
