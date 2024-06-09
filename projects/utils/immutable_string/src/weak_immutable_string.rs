use core::hint::spin_loop;
use core::sync::atomic::Ordering;

use crate::{string_buffer::StringBuffer, ImmutableString};


pub(crate) struct WeakImmutableString {
    content: StringBuffer,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum UpgradeResult {
    Deallocated,
    Strong(ImmutableString),
}

impl WeakImmutableString {
    pub(crate) fn from_buffer(buffer: &StringBuffer) -> Self {
        let clone = unsafe { buffer.clone() };
        Self { content: clone }
    }

    pub(crate) fn upgrade(&self) -> UpgradeResult {
        let strong = self.content.get_strong_counter();
        let mut old_value = strong.load(Ordering::Acquire);
        loop {
            if old_value == 0 {
                return UpgradeResult::Deallocated;
            }

            let cas = strong.compare_exchange_weak(
                old_value,
                old_value+1,
                Ordering::Acquire,
                Ordering::Relaxed);

            match cas {
                Ok(_) => {
                    return UpgradeResult::Strong(
                        ImmutableString::from_buffer(&self.content));
                },
                Err(current_value) => {
                    old_value = current_value;
                }
            }

            spin_loop();
        }
    }
}

impl Clone for WeakImmutableString {
    fn clone(&self) -> Self {
        self.content.inc_weak();
        WeakImmutableString::from_buffer(&self.content)
    }
}

impl Drop for WeakImmutableString {
    fn drop(&mut self) {
        if self.content.dec_weak() != 1 {
            return;
        }

        unsafe { self.content.deallocate(); }
    }
}
