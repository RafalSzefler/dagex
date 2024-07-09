use immutable_string::ImmutableString;

use crate::models::LogDataHolder;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
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
}

pub trait StructuralLogHandler {
    fn handle(&mut self, log: &LogDataHolder);
}

pub trait StructuralLoggerFactoryBuilder {
    type Factory : StructuralLoggerFactory;

    fn add_handler(&mut self, handler: Box<dyn StructuralLogHandler>);

    fn build(self) -> Self::Factory;
}
