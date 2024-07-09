use core::hash::{Hash, Hasher};
use std::{collections::HashMap, time::{Duration, SystemTime}};

use immutable_string::ImmutableString;

use crate::{macros::{readonly, readonly_derive}, traits::LogLevel};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum SLObject {
    Empty,
    LogLevel(SLLogLevel),
    SystemTime(SLSystemTime),
    Duration(SLDuration),
    String(SLString),
    Number(SLNumber),
    Bool(SLBool),
    Array(Box<SLArray>),
    Dict(Box<SLDict>),
}

impl From<LogLevel> for SLObject {
    fn from(value: LogLevel) -> Self { Self::LogLevel(SLLogLevel::new(value)) }
}

impl From<SystemTime> for SLObject {
    fn from(value: SystemTime) -> Self { Self::SystemTime(SLSystemTime::new(value)) }
}

impl From<Duration> for SLObject {
    fn from(value: Duration) -> Self { Self::Duration(SLDuration::new(value)) }
}

impl From<ImmutableString> for SLObject {
    fn from(value: ImmutableString) -> Self { Self::String(SLString::new(value)) }
}

impl From<i64> for SLObject {
    fn from(value: i64) -> Self { Self::Number(SLNumber::new(value)) }
}

impl From<bool> for SLObject {
    fn from(value: bool) -> Self { Self::Bool(SLBool::new(value)) }
}

impl From<Vec<SLObject>> for SLObject {
    fn from(value: Vec<SLObject>) -> Self {
        let arr = SLArray::new(value);
        let boxed = Box::new(arr);
        Self::Array(boxed)
    }
}

impl From<HashMap<ImmutableString, SLObject>> for SLObject {
    fn from(value: HashMap<ImmutableString, SLObject>) -> Self {
        let arr = SLDict::new(value);
        let boxed = Box::new(arr);
        Self::Dict(boxed)
    }
}

readonly_derive!(
    pub struct SLLogLevel {
        value: LogLevel,
    }
);

readonly_derive!(
    pub struct SLSystemTime {
        value: SystemTime,
    }
);

readonly_derive!(
    pub struct SLDuration {
        value: Duration,
    }
);

readonly_derive!(
    pub struct SLString {
        value: ImmutableString,
    }
);

impl From<&str> for SLString {
    fn from(value: &str) -> Self {
        Self::new(ImmutableString::new(value).unwrap())
    }
}

readonly_derive!(
    pub struct SLNumber {
        value: i64,
    }
);

readonly_derive!(
    pub struct SLBool {
        value: bool
    }
);

readonly_derive!(
    pub struct SLArray {
        value: Vec<SLObject>,
    }
);

readonly!(
    pub struct SLDict {
        value: HashMap<ImmutableString, SLObject>,
    }
);

impl PartialEq for SLDict {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for SLDict { }

impl Hash for SLDict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut total_hash = self.value.len() as u64;
        for (key, value) in &self.value {
            let mut fnv1 = fnv1a_hasher::FNV1a32Hasher::new();
            key.hash(&mut fnv1);
            value.hash(&mut fnv1);
            total_hash ^= fnv1.finish();
        }
        state.write_u64(total_hash);
    }
}

impl Clone for SLDict {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl std::fmt::Debug for SLDict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SLDict").field("value", &self.value).finish()
    }
}

#[derive(Default)]
pub struct LogDataHolder {
    log_data: HashMap<ImmutableString, SLObject>,
}

pub mod keys {
    #![allow(non_upper_case_globals)]
    use paste::paste;
    use std::sync::OnceLock;
    use immutable_string::ImmutableString;

    macro_rules! key {
        ( $id: ident ) => {
            paste! {
                pub fn $id() -> ImmutableString {
                    static [< STATIC_ $id >]: OnceLock<ImmutableString>
                        = OnceLock::new();

                    [< STATIC_ $id >].get_or_init(|| {
                        ImmutableString::new(stringify!($id)).unwrap()
                    }).clone()
                }
            }
        };
    }

    key!(created_at);
    key!(log_level);
    key!(logger_name);
    key!(template);
    key!(template_params);
}

impl LogDataHolder {
    pub fn new(
        created_at: SystemTime,
        log_level: LogLevel,
        template: ImmutableString,
        template_params: SLDict) -> Self
    {
        let mut data = HashMap::with_capacity(5);
        data.insert(keys::created_at(), created_at.into());
        data.insert(keys::log_level(), log_level.into());
        data.insert(keys::template(), template.into());
        data.insert(keys::template_params(), SLObject::Dict(Box::new(template_params)));
        Self { log_data: data }
    }

    #[inline(always)]
    pub fn log_data(&self) -> &HashMap<ImmutableString, SLObject> { &self.log_data }

    #[inline(always)]
    pub fn update_data<T>(&mut self, key: ImmutableString, value: T)
        where T: Into<SLObject>
    {
        self.log_data.insert(key, value.into());
    }
}
