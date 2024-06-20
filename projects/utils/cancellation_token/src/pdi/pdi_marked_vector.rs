use super::{PDICollection, PDIItemIndicator};

const INLINE_SIZE: usize = 4;
type InnerVec<T> = smallvec::SmallVec<[Option<T>; INLINE_SIZE]>;

pub struct PDIMarkedVector<T>
{
    inner: InnerVec<T>,
    deleted_count: u32,
}

impl<T> PDIMarkedVector<T> {

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation)]
    fn sweep(&mut self) {
        let inner = &mut self.inner;
        let len = inner.len();
        let threshold = ((len as f64).sqrt().ceil() as u32) + 2;

        if self.deleted_count < threshold {
            return;
        }

        let mut last_idx = 0;
        for idx in 0..len {
            let value = inner[idx].take();
            if value.is_some() {
                inner[last_idx] = value;
                last_idx += 1;
            }
        }

        inner.truncate(last_idx);
        inner.shrink_to_fit();
        self.deleted_count = 0;
    }
}

impl<T> Default for PDIMarkedVector<T> {
    fn default() -> Self {
        Self {
            inner: InnerVec::new(),
            deleted_count: 0,
        }
    }
}

pub struct PDIMarkedIterator<T> {
    pub inner: InnerVec<T>,
    pub current: u32,
    pub end: u32,
}

impl<T> Iterator for PDIMarkedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.current;
        let end = self.end;
        while current < end {
            let value = self.inner[current as usize].take();
            if value.is_some() {
                self.current = current + 1;
                return value;
            }
            current += 1;
        }

        None
    }
}

impl<T> IntoIterator for PDIMarkedVector<T> {
    type Item = T;

    type IntoIter = PDIMarkedIterator<T>;

    #[allow(clippy::cast_possible_truncation)]
    fn into_iter(self) -> Self::IntoIter {
        let end = self.inner.len() as u32;
        PDIMarkedIterator {
            inner: self.inner,
            current: 0,
            end: end,
        }
    }
}

impl<T> PDICollection for PDIMarkedVector<T>
{
    type Item = T;
    type Indicator<'a> = PDIMarkedItemIndicator<'a, T> where Self: 'a;

    fn push(&mut self, item: <Self as PDICollection>::Item) -> Self::Indicator<'_> {
        let idx = self.inner.len();
        assert!(idx < (i32::MAX as usize), "Too many items in PDIMarkedVector.");

        self.inner.push(Some(item));
        Self::Indicator {
            vec: self,
            idx: idx,
        }
    }

    #[inline(always)]
    fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.inner, &mut other.inner);
    }
}


pub struct PDIMarkedItemIndicator<'a, T> {
    vec: &'a mut PDIMarkedVector<T>,
    idx: usize,
}


impl<'a, T> PDIItemIndicator<'a> for PDIMarkedItemIndicator<'a, T> {
    type CollectionType = PDIMarkedVector<T>;

    #[inline(always)]
    fn remove(self) {
        self.vec.inner[self.idx].take();
        self.vec.deleted_count += 1;
        self.vec.sweep();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    impl<T> PDIMarkedVector<T> {
        #[inline(always)]
        fn len(&self) -> usize {
            self.inner.len() - (self.deleted_count as usize)
        }
    }

    #[test]
    fn test_simple_call() {
        let mut pdi = PDIMarkedVector::default();
        pdi.push(5);
        let indicator = pdi.push(7);
        indicator.remove();
        pdi.push(17);

        let data = Vec::from_iter(pdi.into_iter());
        assert_eq!(&data, &[5, 17]);
    }

    #[test]
    fn test_more_calls() {
        let mut pdi = PDIMarkedVector::default();
        pdi.push(5);
        {
            let indicator = pdi.push(7);
            indicator.remove();
        }
        pdi.push(17);

        let data = Vec::from_iter(pdi.into_iter());
        assert_eq!(&data, &[5, 17]);
    }

    #[test]
    fn test_even_more_calls() {
        let mut pdi = PDIMarkedVector::default();
        pdi.push(5);
        
        for idx in 0..10000 {
            assert_eq!(pdi.len(), 1);
            let removable = pdi.push(idx);
            assert_eq!(removable.vec.len(), 2);
            removable.remove();
        }
    }
}
