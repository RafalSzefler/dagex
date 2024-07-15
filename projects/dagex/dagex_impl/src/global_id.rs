use core::sync::atomic::{AtomicI32, Ordering};

/// Represents a global identifier, unique during process lifetime.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct GlobalId {
    id: i32,
}

static _ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);

impl GlobalId {

    /// Creates a new unique [`GlobalId`]. Thread safe.
    #[inline(always)]
    pub fn generate_next() -> Self {
        let id = _ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self { id }
    }
}

impl From<GlobalId> for i32 {
    #[inline(always)]
    fn from(value: GlobalId) -> Self {
        value.id
    }
}
