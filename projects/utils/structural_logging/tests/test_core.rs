use std::{collections::HashMap, sync::{Arc, Mutex}, time::SystemTime};

use immutable_string::ImmutableString;
use structural_logging::{core::CoreLoggerFactoryBuilder, models::{LogDataHolder, SLDict, SLObject}, traits::{LogLevel, StructuralLog, StructuralLogHandler, StructuralLogger, StructuralLoggerFactory, StructuralLoggerFactoryBuilder}};

#[derive(Default)]
pub struct TestHandler {
    logger_names: Arc<Mutex<Vec<ImmutableString>>>,
}

impl TestHandler {
    pub fn new(logger_names: Arc<Mutex<Vec<ImmutableString>>>) -> Self {
        Self { logger_names }
    }
}

impl StructuralLogHandler for TestHandler {
    fn handle(&mut self, log: &LogDataHolder) {
        let key = ImmutableString::new("logger_name").unwrap();
        let name_object = &log.log_data()[&key];
        let name = match name_object {
            SLObject::String(value) => value.value(),
            _ => panic!("Invalid logger_name"),
        };
        let mut guard = self.logger_names.lock().unwrap();
        guard.push(name.clone());
    }
}

pub struct TestLog { }

impl StructuralLog for TestLog {
    fn log_data(&self) -> LogDataHolder {
        LogDataHolder::new(
            SystemTime::now(),
            LogLevel::Info,
            ImmutableString::new("xyz").unwrap(),
            SLDict::new(HashMap::new()))
    }
}

#[test]
fn test_core() {
    let names = Arc::new(Mutex::new(Vec::new()));

    {
        let handler = TestHandler::new(names.clone());
        let mut builder = CoreLoggerFactoryBuilder::default();
        builder.add_handler(Box::new(handler));
        let factory = builder.build();
        let logger1 = factory.create_from_str("misc");
        let logger2 = factory.create_from_str("abc");
        let logger3 = factory.create_from_str("X");

        logger1.log(TestLog {});
        logger2.log(TestLog {});
        logger3.log(TestLog {});
        logger1.log(TestLog {});
    }

    let strings: Vec<String>;

    {
        let guard = names.lock().unwrap();
        strings = guard.iter().map(|imm| imm.as_str().to_owned()).collect();
    }

    let expected = ["misc", "abc", "X", "misc"];
    let expected: Vec<String> = expected.into_iter().map(ToOwned::to_owned).collect();
    assert_eq!(strings, expected);
}
