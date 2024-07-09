use crate::traits::StructuralLogger;

pub struct CoreLogger {

}

impl StructuralLogger for CoreLogger {
    fn log<T>(&self, log: T) where T : crate::traits::StructuralLog {
        todo!()
    }
}