use crate::traits::StructuralLoggerFactory;

use super::CoreLogger;

pub struct CoreLoggerFactory {

}

impl StructuralLoggerFactory for CoreLoggerFactory {
    type Logger = CoreLogger;

    fn create(&self, name: &immutable_string::ImmutableString) -> Self::Logger {
        todo!()
    }
}