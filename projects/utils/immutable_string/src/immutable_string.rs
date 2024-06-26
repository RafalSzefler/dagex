use core::hash::{Hash, Hasher};
use core::fmt::{Debug, Display, Formatter};
use core::str::from_utf8_unchecked;
use core::sync::atomic::Ordering;

use crate::types::{AtomicUnderlyingType, HashType, LengthType};
use crate::weak_immutable_string::UpgradeResult;
use crate::{
    buffer::Buffer,
    cache::get_cache,
    string_buffer::StringBuffer,
    types::MAX_LENGTH,
    weak_immutable_string::WeakImmutableString,
    ConstructionError,
};

/// Represents a string that cannot change during its lifetime. The struct
/// is thread safe, and uses a global cache of strings internally, so creating
/// it out of the same string is fast and memory efficient.
pub struct ImmutableString {
    content: StringBuffer,
}

/// Stores internal information about [`ImmutableString`].
#[derive(Clone)]
pub struct ImmutableStringInfo {
    pub strong_count: AtomicUnderlyingType,
    pub hash: HashType,
    pub length: LengthType,

    #[cfg(test)]
    pub weak_count: AtomicUnderlyingType,
}

#[cfg(feature="ctor")]
mod empty_impl
{
    use ctor::ctor;

    use crate::string_buffer::StringBuffer;

    use super::ImmutableString;

    #[ctor]
    static _EMPTY: ImmutableString
        = ImmutableString::from_buffer(&StringBuffer::new("").unwrap());

    #[inline(always)]
    pub(super) fn get_empty() -> &'static ImmutableString { &_EMPTY }
}

#[cfg(not(feature="ctor"))]
mod empty_impl
{
    use std::sync::OnceLock;
    use crate::string_buffer::StringBuffer;

    use super::ImmutableString;

    static _EMPTY: OnceLock<ImmutableString> = OnceLock::new();

    #[inline(always)]
    pub(super) fn get_empty() -> &'static ImmutableString {
        _EMPTY.get_or_init(|| { ImmutableString::from_buffer(&StringBuffer::new("").unwrap()) })
    }
}

impl ImmutableString {
    #[inline(always)]
    pub const fn get_max_length() -> usize { MAX_LENGTH }

    #[inline(always)]
    pub fn empty() -> &'static ImmutableString { empty_impl::get_empty() }

    /// Creates new [`ImmutableString`] from `text`. If given `text` was
    /// already used, it will retrieve it from a global cache without
    /// any copies. Otherwise it will copy `text`, construct new
    /// [`ImmutableString`] and cache the result for later usage.
    /// 
    /// # Errors
    ///
    /// * [`ConstructionError::LengthTooBig`] if `text.len()` exceeds
    /// [`ImmutableString::get_max_length()`].
    /// 
    /// * [`ConstructionError::AllocationError`] if is not able
    /// to allocate memory for new `ImmutableString`.
    pub fn get(text: &str) -> Result<Self, ConstructionError> {
        if text.is_empty() {
            return Ok(Self::empty().clone());
        }

        if text.len() > ImmutableString::get_max_length() {
            return Err(ConstructionError::LengthTooBig);
        }

        {
            let raw_buffer = Buffer::from(text);
            let read 
                = unsafe { get_cache().read().unwrap_unchecked() };
            if let Some(weak) = read.get(&raw_buffer) {
                match weak.upgrade() {
                    UpgradeResult::Strong(instance) => {
                        return Ok(instance);
                    },
                    UpgradeResult::Deallocated => { },
                }
            }
        }

        let string_buffer: StringBuffer = match StringBuffer::new(text) {
            Ok(result_buffer) => { result_buffer },
            Err(err) => {
                return Err(err);
            }
        };

        let key = Buffer::from(&string_buffer);
        let imm = ImmutableString::from_buffer(&string_buffer);
        let weak 
            = ImmutableString::downgrade(&imm);

        {
            let mut write 
                = unsafe { get_cache().write().unwrap_unchecked() };
            write.insert(key, weak);
        }

        return Ok(imm);
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe {
            from_utf8_unchecked(self.content.as_slice())
        }
    }

    #[inline(always)]
    pub fn len(&self) -> LengthType {
        self.content.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_info(&self) -> ImmutableStringInfo {
        ImmutableStringInfo {
            strong_count: self.content.get_strong_counter()
                .load(Ordering::Relaxed),
            hash: self.content.get_hash(),
            length: self.len(),

            #[cfg(test)]
            weak_count: self.content.get_weak_counter()
                .load(Ordering::Relaxed),
        }
    }

    #[inline(always)]
    pub(crate) fn downgrade(instance: &ImmutableString) -> WeakImmutableString {
        instance.content.inc_weak();
        WeakImmutableString::from_buffer(&instance.content)
    }

    #[inline(always)]
    pub(crate) fn from_buffer(buffer: &StringBuffer) -> Self {
        let clone = unsafe { buffer.clone() };
        Self { content: clone }
    }
}

impl Default for ImmutableString {
    fn default() -> Self { Self::empty().clone() }
}

impl Clone for ImmutableString {
    /// This method doesn't actually clone the string, but it
    /// increments the internal reference counter instead. This operation
    /// is thread safe and allows sharing of the [`ImmutableString`], without
    /// the need of copy and allocation.
    fn clone(&self) -> Self {
        self.content.inc_strong();
        Self::from_buffer(&self.content)
    }
}

impl Drop for ImmutableString {
    fn drop(&mut self) {
        if self.content.dec_strong() != 1 {
            return;
        }

        let key = Buffer::from(&self.content);
        let _removed_weak: Option<WeakImmutableString>;

        {
            let mut write 
                = unsafe { get_cache().write().unwrap_unchecked() };
            _removed_weak = write.remove(&key);
        }

        // We want both _removed_weak and _local_weak to be dropped
        // outside of lock.
        let _local_weak = WeakImmutableString
            ::from_buffer(&self.content);
    }
}

impl PartialEq for ImmutableString {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Eq for ImmutableString { }

impl Hash for ImmutableString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.content.get_hash());
    }
}

impl Display for ImmutableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for ImmutableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let info = self.get_info();
        let mut debug = f.debug_struct("ImmutableString");
        debug
            .field("strong_count", &info.strong_count)
            .field("length", &info.length)
            .field("hash", &info.hash)
            .field("content", &self.as_str());

        #[cfg(test)]
        {
            debug.field("weak_count", &info.weak_count);
        }

        debug.finish()
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]
#[cfg(test)]
mod tests {
    use std::{sync::{Mutex, MutexGuard, OnceLock}, time::Duration};

    use super::*;

    impl UpgradeResult {
        fn has_value(&self) -> bool { *self != UpgradeResult::Deallocated }
    }

    impl ImmutableString {
        fn as_ptr(&self) -> *const u8 { self.content.as_ptr() }
    }

    fn get_cache_size() -> i32 {
        let read = unsafe {
            get_cache().read().unwrap_unchecked()
        };
        read.len() as i32
    }

    fn get_test_pair(text: &str) -> (String, String) {
        let first = String::from(text);
        let second = String::from(text);
        (first, second)
    }

    static _LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_lock() -> MutexGuard<'static, ()> {
        _LOCK.get_or_init(|| { Mutex::default() })
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    #[test]
    fn test_immutable_string() {
        let _guard = test_lock();
        let (first, second) = get_test_pair("test");
        let first_str = first.as_str();
        let second_str = second.as_str();
        assert!(!std::ptr::eq(first_str, second_str));
        assert_eq!(get_cache_size(), 0);
        let string1 = ImmutableString::get(first_str)
            .unwrap();
        assert_eq!(get_cache_size(), 1);
        let string2 = ImmutableString::get(second_str)
            .unwrap();
        assert_eq!(get_cache_size(), 1);

        assert_eq!(string1, string2);
        assert_eq!(
            string1.content.get_buffer_size(),
            string2.content.get_buffer_size());
        assert_eq!(string1.content.get_buffer_size(), 12);
        assert_eq!(string1.content, string2.content);
        assert_eq!(string1.content.get_hash(), string2.content.get_hash());
        assert!(std::ptr::eq(string1.as_ptr(), string2.as_ptr()));
        drop(string2);
        assert_eq!(get_cache_size(), 1);
        drop(string1);
        assert_eq!(get_cache_size(), 0);
    }

    #[cfg(any(target_arch="x86_64", target_arch="aarch64"))]
    #[test]
    fn test_immutable_string_size() {
        use rand::Rng;
        const PREFIX_SIZE: i32 = 8;
        const ALIGNMENT: i32 = 4;
        const ALPHABET: &[u8] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();

        let _guard = test_lock();
        assert_eq!(get_cache_size(), 0);

        {
            // Empty string is special - it is not stored inside cache, but
            // rather statically kept for the lifetime of app.
            let imm = ImmutableString::get("").unwrap();
            assert_eq!(imm.content.get_buffer_size(), PREFIX_SIZE);
            assert_eq!(get_cache_size(), 0);
            let info = imm.get_info();
            assert_eq!(info.strong_count, 2);
            assert_eq!(info.weak_count, 1);
            assert_eq!(info.length, 0);
            assert_eq!(info.hash, 0);
            let weak 
                = ImmutableString::downgrade(&imm);
            assert!(weak.upgrade().has_value());
            drop(imm);
            assert!(weak.upgrade().has_value());
        }

        let test_cases = [
            "x",
            "xy",
            "xyz",
            "xyze",
            "xyzeg",
            "xyzegs",
            "xyzegss",
            "xyzegssa",
            "xyzegssas",
        ];

        let test_text = |case: &str| {
            let text_len = case.len() as i32;
            let pad = (((text_len - 1) / ALIGNMENT) + 1) * ALIGNMENT;
            let expected_size = PREFIX_SIZE + pad;
            let imm = ImmutableString::get(case)
                .unwrap();
            assert_eq!(imm.content.get_buffer_size(), expected_size);
            let info = imm.get_info();
            assert_eq!(info.strong_count, 1);
            assert_eq!(info.weak_count, 2);
            let weak 
                = ImmutableString::downgrade(&imm);
            assert!(weak.upgrade().has_value());
            assert_eq!(get_cache_size(), 1);
            let clone = imm.clone();
            assert_eq!(imm.get_info().strong_count, 2);
            assert_eq!(imm.get_info().weak_count, 3);
            assert_eq!(get_cache_size(), 1);
            drop(clone);
            assert!(weak.upgrade().has_value());
            assert_eq!(get_cache_size(), 1);
            drop(imm);
            assert!(!weak.upgrade().has_value());
        };

        for text in test_cases {
            test_text(text);
        }

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let size = rng.gen_range(10..100);
            let mut buffer = Vec::<u8>::with_capacity(size);
            for _ in 0..size {
                let index = rng.gen_range(0..ALPHABET.len());
                buffer.push(ALPHABET[index]);
            }
            let text = unsafe { from_utf8_unchecked(buffer.as_slice()) };
            test_text(text);
        }

        assert_eq!(get_cache_size(), 0);
    }

    #[test]
    fn test_parallel_new_immutable_string() {
        const THRAED_COUNT: usize = 100;

        let _guard = test_lock();
        let imm = ImmutableString::get("foo").unwrap();

        let mut threads 
            = Vec::<std::thread::JoinHandle<()>>::with_capacity(THRAED_COUNT);
        for _ in 0..THRAED_COUNT {
            threads.push(std::thread::spawn(|| {
                let copy 
                    = ImmutableString::get("foo").unwrap();
                std::thread::sleep(Duration::from_millis(50));
                drop(copy);
            }));
        }

        assert_eq!(get_cache_size(), 1);
        drop(imm);
        std::thread::sleep(Duration::from_millis(25));
        assert_eq!(get_cache_size(), 1);
        for th in threads {
            th.join().unwrap();
        }
        assert_eq!(get_cache_size(), 0);
    }

    #[test]
    fn test_parallel_clone_immutable_string() {
        const THRAED_COUNT: usize = 100;

        let _guard = test_lock();
        let imm = ImmutableString::get("foo").unwrap();

        let mut threads 
            = Vec::<std::thread::JoinHandle<()>>::with_capacity(THRAED_COUNT);
        for _ in 0..THRAED_COUNT {
            let imm_clone = imm.clone();
            threads.push(std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                drop(imm_clone);
            }));
        }

        assert_eq!(get_cache_size(), 1);
        drop(imm);
        assert_eq!(get_cache_size(), 1);
        for th in threads {
            th.join().unwrap();
        }
        assert_eq!(get_cache_size(), 0);
    }

    #[test]
    fn test_parallel_mixed_immutable_string() {
        const THRAED_COUNT: usize = 100;

        let _guard = test_lock();
        let imm = ImmutableString::get("foo").unwrap();

        let mut threads 
            = Vec::<std::thread::JoinHandle<()>>::with_capacity(THRAED_COUNT);
        for _ in 0..(THRAED_COUNT/2) {
            let imm_clone = imm.clone();
            threads.push(std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                drop(imm_clone);
            }));
            threads.push(std::thread::spawn(|| {
                let copy 
                    = ImmutableString::get("foo").unwrap();
                std::thread::sleep(Duration::from_millis(50));
                drop(copy);
            }));
        }

        assert_eq!(get_cache_size(), 1);
        drop(imm);
        assert_eq!(get_cache_size(), 1);
        for th in threads {
            th.join().unwrap();
        }
        assert_eq!(get_cache_size(), 0);
    }
}
