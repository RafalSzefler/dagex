use std::{collections::HashMap, sync::RwLock};

use crate::{buffer::Buffer, weak_immutable_string::WeakImmutableString};

pub(crate) type CacheType = RwLock<HashMap<Buffer, WeakImmutableString>>;


#[cfg(feature="ctor")]
mod cache_impl
{
    use std::{collections::HashMap, sync::RwLock};

    use ctor::ctor;

    use super::CacheType;

    #[ctor]
    static _CACHE: CacheType = RwLock::new(HashMap::new());

    #[inline(always)]
    pub(super) fn get() -> &'static CacheType { &_CACHE }
}

#[cfg(not(feature="ctor"))]
mod cache_impl
{
    use std::{collections::HashMap, sync::{OnceLock, RwLock}};

    use super::CacheType;

    static _CACHE: OnceLock<CacheType> = OnceLock::new();

    #[inline(always)]
    pub(super) fn get() -> &'static CacheType {
        _CACHE.get_or_init(|| { RwLock::new(HashMap::new()) })
    }
}

#[inline(always)]
pub(crate) fn get_cache() -> &'static CacheType { cache_impl::get() }
