use std::sync::Arc;

use immutable_string::ImmutableString;

use crate::models::LogDataHolder;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl Default for LogLevel {
    fn default() -> Self { Self::Debug }
}

pub trait StructuralLog {
    fn log_data(&self) -> LogDataHolder;
}

pub trait StructuralLogger {
    fn log<T>(&self, log: T) where T : StructuralLog;
}

pub trait StructuralLoggerFactory {
    type Logger : StructuralLogger;

    fn create(&self, name: &ImmutableString) -> Self::Logger;

    fn create_from_str(&self, name: &str) -> Self::Logger {
        let imm = ImmutableString::new(name)
            .expect("StructuralLoggerFactory - create_from_str() fail on new ImmutableString");
        self.create(&imm)
    }
}

pub trait StructuralLogHandler : Sync + Send {
    fn handle(&self, log: &LogDataHolder);
}

pub trait StructuralLoggerFactoryBuilder {
    type Factory : StructuralLoggerFactory;

    fn add_handler(&mut self, handler: Arc<dyn StructuralLogHandler>);

    fn set_log_level(&mut self, log_level: LogLevel);

    fn build(self) -> Self::Factory;
}
