use core::hash::{Hash, Hasher};
use std::{cell::UnsafeCell, collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, OnceLock}, time::{Duration, SystemTime, UNIX_EPOCH}};

use immutable_string::ImmutableString;

use crate::{macros::{readonly, readonly_derive}, template::Template, traits::LogLevel};

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

pub struct LogDataHolder {
    empty: bool,
    template: Template,
    log_level: LogLevel,
    created_at: SystemTime,
    template_params: HashMap<ImmutableString, SLObject>,
    additional_data: UnsafeCell<HashMap<ImmutableString, SLObject>>,
    additional_data_initialized: AtomicBool,
}

impl Default for LogDataHolder {
    fn default() -> Self {
        Self {
            empty: true,
            log_level: LogLevel::default(),
            created_at: UNIX_EPOCH,
            template: Template::default(),
            template_params: HashMap::default(),
            additional_data: UnsafeCell::default(),
            additional_data_initialized: AtomicBool::default(),
        }
    }
}

static CREATED_AT: OnceLock<ImmutableString> = OnceLock::new();
fn key_created_at() -> &'static ImmutableString {
    CREATED_AT.get_or_init(|| { ImmutableString::new("created_at").unwrap() })
}

static LOG_LEVEL: OnceLock<ImmutableString> = OnceLock::new();
fn key_log_level() -> &'static ImmutableString {
    LOG_LEVEL.get_or_init(|| { ImmutableString::new("log_level").unwrap() })
}

impl LogDataHolder {
    pub fn new(
        log_level: LogLevel,
        template: Template,
        template_params: HashMap<ImmutableString, SLObject>) -> Self
    {
        Self {
            empty: false,
            template: template,
            created_at: SystemTime::now(),
            log_level: log_level,
            template_params: template_params,
            additional_data: UnsafeCell::default(),
            additional_data_initialized: AtomicBool::default(),
        }
    }

    #[inline(always)]
    pub fn template(&self) -> &Template { &self.template }

    #[inline(always)]
    pub fn created_at(&self) -> SystemTime { self.created_at }

    #[inline(always)]
    pub fn log_level(&self) -> LogLevel { self.log_level }

    #[inline(always)]
    pub fn template_params(&self) -> &HashMap<ImmutableString, SLObject> { &self.template_params }

    #[inline(always)]
    pub fn additional_data(&self) -> &HashMap<ImmutableString, SLObject> {
        self.additional_data_mut()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool { self.empty }

    #[inline(always)]
    pub fn update_data<T>(&mut self, key: ImmutableString, value: T)
        where T: Into<SLObject>
    {
        self.additional_data_mut().insert(key, value.into());
    }

    #[allow(clippy::mut_from_ref)]
    fn additional_data_mut(&self) -> &mut HashMap<ImmutableString, SLObject> {
        let additional_data = unsafe {
            &mut *self.additional_data.get()
        };

        let result = self.additional_data_initialized.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed);
        
        if result.is_ok() {
            let mut new_additional_data = HashMap::with_capacity(4);
            new_additional_data.insert(key_created_at().clone(), self.created_at.into());
            new_additional_data.insert(key_log_level().clone(), self.log_level.into());
            *additional_data = new_additional_data;
        }

        additional_data
    }
}
