use std::{hash::Hasher, sync::{Arc, OnceLock}};

use dagex::raf_array::immutable_string::ImmutableString;
use raf_structural_logging::{
    core::{CoreLoggerFactory, CoreLoggerFactoryBuilder},
    traits::StructuralLoggerFactoryBuilder};
use raf_structural_logging_console::ConsoleHandler;

static DEFAULT_LOGGER_FACTORY: OnceLock<Arc<CoreLoggerFactory>> = OnceLock::new();

pub fn build_default_logger_factory() -> Arc<CoreLoggerFactory> {
    DEFAULT_LOGGER_FACTORY.get_or_init(|| {
        let console_handler = Arc::new(ConsoleHandler);
        let mut builder = CoreLoggerFactoryBuilder::default();
        builder.add_handler(console_handler);
        Arc::new(builder.build())
    }).clone()
}

/// Build name for logger based on `prefix` and some hashable input/data.
/// 
/// # Panics
/// When can't construct [`ImmutableString`] for whatever reason.
pub fn build_logger_name<T: core::hash::Hash>(prefix: &str, data: &T) -> ImmutableString {
    const INLINE_SIZE: usize = 127;
    const MAX_PREFIX_SIZE: usize = INLINE_SIZE - 23;
    let prefix_len = prefix.len();
    assert!(prefix_len < MAX_PREFIX_SIZE, "prefix.len() cannot exceed {MAX_PREFIX_SIZE}.");
    let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
    data.hash(&mut hasher);
    let mut value = hasher.finish();
    let mut buffer = [0u8; INLINE_SIZE];

    buffer[0..prefix_len].copy_from_slice(prefix.as_bytes());
    buffer[prefix_len] = b'-';
    let nobuffer = &mut buffer[(prefix_len+1)..INLINE_SIZE];
    let mut offset = 0;
    loop {
        nobuffer[offset] = (value % 10) as u8;
        offset += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    nobuffer[0..offset].reverse();
    let complete_slice = &buffer[0..(prefix_len+1+offset)];
    let txt = unsafe { core::str::from_utf8_unchecked(complete_slice) };
    ImmutableString::new(txt).unwrap()
}
