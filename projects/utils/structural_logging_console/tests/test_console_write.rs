use std::{collections::HashMap, time::{Duration, SystemTime}};

use immutable_string::ImmutableString;
use structural_logging::{models::{LogDataHolder, SLDict, SLObject}, traits::{LogLevel, StructuralLogHandler}};
use structural_logging_console::ConsoleHandler;

#[test]
fn test1() {
    let now = SystemTime::now();
    let log_level = LogLevel::Info;
    let test = ImmutableString::new("test").unwrap();
    let sldict = SLDict::new(HashMap::new());
    let log_data = LogDataHolder::new(
        now.clone(),
        log_level,
        test.clone(),
        sldict.clone());
    
    let dict = log_data.log_data();

    let get = |txt: &str| {
        let imm = ImmutableString::new(txt).unwrap();
        &dict[&imm]
    };

    assert_eq!(get("created_at"), &now.into());
    assert_eq!(get("log_level"), &log_level.into());
    assert_eq!(get("template"), &test.into());
    assert_eq!(get("template_params"), &SLObject::Dict(Box::new(sldict)));
}


#[test]
fn test2() {
    let mut handler = ConsoleHandler::default();

    let now = SystemTime::now();
    let log_level = LogLevel::Info;
    let test = ImmutableString::new("[{created_at}] [{log_level}] [{dur}] {baz} = {xyz}... {arr}").unwrap();
    let sldict = SLDict::new(HashMap::new());
    let mut log_data = LogDataHolder::new(
        now.clone(),
        log_level,
        test.clone(),
        sldict.clone());
    
    let key = ImmutableString::new("dur").unwrap();
    log_data.update_data(key, Duration::from_millis(12100));
    let key = ImmutableString::new("baz").unwrap();
    log_data.update_data(key, 256i64);
    let key = ImmutableString::new("xyz").unwrap();
    log_data.update_data(key, false);

    let key = ImmutableString::new("arr").unwrap();
    let vec: Vec<SLObject> = vec![true.into(), (-15i64).into(), LogLevel::Error.into()];
    log_data.update_data(key, vec);
    
    handler.handle(&log_data);


    let now2 = SystemTime::now();
    let log_level2 = LogLevel::Info;
    let test2 = ImmutableString::new("[{created_at}] this is dict: {dct}  ").unwrap();
    let sldict2 = SLDict::new(HashMap::new());
    let mut log_data2 = LogDataHolder::new(
        now2.clone(),
        log_level2,
        test2.clone(),
        sldict2.clone());
    
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
    handler.handle(&log_data2);
}
