use std::sync::{Arc, OnceLock};

use structural_logging::{core::{CoreLoggerFactory, CoreLoggerFactoryBuilder}, traits::StructuralLoggerFactoryBuilder};
use structural_logging_console::ConsoleHandler;

static DEFAULT_LOGGER_FACTORY: OnceLock<Arc<CoreLoggerFactory>> = OnceLock::new();

pub fn build_default_logger_factory() -> Arc<CoreLoggerFactory> {
    DEFAULT_LOGGER_FACTORY.get_or_init(|| {
        let console_handler = Arc::new(ConsoleHandler);
        let mut builder = CoreLoggerFactoryBuilder::default();
        builder.add_handler(console_handler);
        Arc::new(builder.build())
    }).clone()
}
