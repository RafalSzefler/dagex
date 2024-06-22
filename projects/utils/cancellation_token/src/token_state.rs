#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenState {
    Ok,
    IsCancelled,
    NotCancellable,
}
