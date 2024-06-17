#[derive(Debug, Clone, Copy)]
pub struct IsCancelled {
    _marker: (),
}

impl Default for IsCancelled {
    fn default() -> Self { Self { _marker: () } }
}
