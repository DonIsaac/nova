use std::ops::{Index, IndexMut};

use crate::heap::{CompactionLists, HeapMarkAndSweep as _};

use super::{SubspaceKey, SubspaceStorage};

/// An isolated subspace.
///
/// Isolated subspaces only contain heap entries of a single type.
#[derive(Debug, Default)]
pub(crate) struct IsoSubspace<T: SubspaceKey>(Vec<Option<T::HeapData>>);

impl<T: SubspaceKey> IsoSubspace<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub(crate) fn sweep(&mut self, compactions: &CompactionLists, bits: &[bool]) {
        assert_eq!(self.0.len(), bits.len());
        let mut iter = bits.iter();
        self.0.retain_mut(|item| {
            let do_retain = iter.next().unwrap();
            if *do_retain {
                item.sweep_values(compactions);
                true
            } else {
                false
            }
        });
    }

    pub fn get(&self, index: T) -> Option<&T::HeapData> {
        self.0
            .get(index.get_index())
            .expect("key is out of bounds")
            .as_ref()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: SubspaceKey> Index<T> for IsoSubspace<T> {
    type Output = T::HeapData;
    fn index(&self, index: T) -> &Self::Output {
        self.get(index).expect("Heap slot empty")
    }
}

impl<T: SubspaceKey> IndexMut<T> for IsoSubspace<T> {
    fn index_mut(&mut self, index: T) -> &mut <Self as Index<T>>::Output {
        self.0
            .get_mut(index.get_index())
            .expect("key is out of bounds")
            .as_mut()
            .expect("Heap slot empty")
    }
}

impl<T: SubspaceKey> SubspaceStorage<T, T::HeapData> for IsoSubspace<T> {
    #[inline]
    fn size(&self) -> usize {
        self.0.len() * size_of::<T::HeapData>()
    }

    fn alloc(&mut self, value: T::HeapData) -> T {
        self.0.push(Some(value));
        T::from_raw_index(self.0.len())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
