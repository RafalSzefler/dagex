use std::sync::Arc;

use raf_immutable_string::ImmutableString;

use crate::{template::get_logger_name_key, traits::{LogLevel, StructuralLog, StructuralLogger}};

use super::background_worker::BackgroundWorker;

pub struct CoreLogger {
    log_level: LogLevel,
    name: ImmutableString,
    worker: Arc<BackgroundWorker>,
}

impl CoreLogger {
    pub(super) fn new(
        log_level: LogLevel,
        name: ImmutableString,
        worker: Arc<BackgroundWorker>) -> Self
    {
        Self { log_level, name, worker }
    }
}


impl StructuralLogger for CoreLogger {
    fn log<T>(&self, log: T) where T : StructuralLog {
        let mut log_data = log.log_data();
        let log_level = log_data.log_level() as i32;
        let self_log_level = self.log_level as i32;
        if log_level < self_log_level {
            return;
        }
        log_data.update_data(get_logger_name_key().clone(), self.name.clone());
        self.worker.send_log(log_data);
    }
}
