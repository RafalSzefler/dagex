use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use immutable_string::ImmutableString;
use structural_logging::{core::CoreLoggerFactoryBuilder, models::{LogDataHolder, SLObject}, template::TemplateBuilder, traits::{LogLevel, StructuralLog, StructuralLogHandler, StructuralLogger, StructuralLoggerFactory, StructuralLoggerFactoryBuilder}};

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
        let name_object = &log.additional_data()[&key];
        let name = match name_object {
            SLObject::String(value) => value.value(),
            _ => panic!("Invalid logger_name"),
        };
        let mut guard = self.logger_names.lock().unwrap();
        guard.push(name.clone());
    }
}

static TMPL_BUILDER: OnceLock<TemplateBuilder> = OnceLock::new();
pub fn tmpl_builder() -> &'static TemplateBuilder {
    TMPL_BUILDER.get_or_init(Default::default)
}

pub struct TestLog { }

impl StructuralLog for TestLog {
    fn log_data(&self) -> LogDataHolder {
        let tmpl = tmpl_builder().create("xyz");
        LogDataHolder::new(
            LogLevel::Info,
            tmpl,
            HashMap::new())
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
