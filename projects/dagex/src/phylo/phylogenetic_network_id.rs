use core::sync::atomic::{AtomicI32, Ordering};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct PhylogeneticNetworkId {
    id: i32,
}

static _ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);

impl PhylogeneticNetworkId {    
    #[inline(always)]
    pub fn generate_next() -> Self {
        let id = _ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self::from(id)
    }

    #[inline(always)]
    pub fn get_numeric_id(&self) -> i32 {
        self.id
    }
}

impl From<i32> for PhylogeneticNetworkId {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}
