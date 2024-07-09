use std::{collections::HashMap, sync::OnceLock, time::{Duration, SystemTime}};

use immutable_string::ImmutableString;
use structural_logging::{models::{LogDataHolder, SLObject}, template::TemplateBuilder, traits::{LogLevel, StructuralLogHandler}};
use structural_logging_console::ConsoleHandler;

static TMPL_BUILDER: OnceLock<TemplateBuilder> = OnceLock::new();
pub fn tmpl_builder() -> &'static TemplateBuilder {
    TMPL_BUILDER.get_or_init(Default::default)
}

#[test]
fn test1() {
    let now = SystemTime::now();
    let log_level = LogLevel::Info;
    let template = tmpl_builder().create("test");
    let template_clone = template.clone();
    let sldict = HashMap::new();
    let log_data = LogDataHolder::new(
        log_level,
        template,
        sldict.clone());

    assert!(log_data.created_at() >= now);
    assert!(log_data.created_at() <= now + Duration::from_millis(100));
    assert_eq!(log_data.log_level(), log_level);
    assert_eq!(log_data.template(), &template_clone);
    assert_eq!(log_data.template_params(), &sldict);
}


#[test]
fn test2() {
    let log_level = LogLevel::Info;
    let template = tmpl_builder().create("[{created_at}] [{log_level}] [{dur}] {baz} = {xyz}... {arr}");
    let template_clone = template.clone();
    let mut log_data = LogDataHolder::new(
        log_level,
        template_clone,
        HashMap::new());
    
    let key = ImmutableString::new("dur").unwrap();
    log_data.update_data(key, Duration::from_millis(12100));
    let key = ImmutableString::new("baz").unwrap();
    log_data.update_data(key, 256i64);
    let key = ImmutableString::new("xyz").unwrap();
    log_data.update_data(key, false);

    let key = ImmutableString::new("arr").unwrap();
    let vec: Vec<SLObject> = vec![true.into(), (-15i64).into(), LogLevel::Error.into()];
    log_data.update_data(key, vec);
    
    ConsoleHandler.handle(&log_data);


    let log_level2 = LogLevel::Info;
    let template2 = tmpl_builder().create("[{created_at}] this is dict: {dct}  ");
    let mut log_data2 = LogDataHolder::new(
        log_level2,
        template2,
        HashMap::new());
    
    let mut map = HashMap::<ImmutableString, SLObject>::new();
    macro_rules! insert {
        ( $key: expr, $value: expr ) => {
            {
                let k = ImmutableString::new($key).unwrap();
                let v = { $value };
                map.insert(k, v.into());
            }
        };
    }
    insert!("debug", LogLevel::Debug);
    insert!("info", LogLevel::Info);
    insert!("warn", LogLevel::Warning);
    insert!("err", LogLevel::Error);
    insert!("i64", 1234215);
    insert!("bazzzzz true", true);
    insert!("bazzzzz false", false);

    let key = ImmutableString::new("dct").unwrap();
    log_data2.update_data(key, map);
    ConsoleHandler.handle(&log_data2);
}
