use std::sync::Arc;

use crate::traits::{StructuralLogHandler, StructuralLoggerFactoryBuilder};

use super::{background_worker::BackgroundWorker, CoreLoggerFactory};

#[derive(Default)]
pub struct CoreLoggerFactoryBuilder {
    handlers: Vec<Box<dyn StructuralLogHandler>>,
}

impl StructuralLoggerFactoryBuilder for CoreLoggerFactoryBuilder {
    type Factory = CoreLoggerFactory;

    fn add_handler(&mut self, handler: Box<dyn StructuralLogHandler>) {
        self.handlers.push(handler);
    }

    fn build(self) -> Self::Factory {
        let worker = BackgroundWorker::new(self.handlers);
        CoreLoggerFactory::new(Arc::new(worker))
    }
}
