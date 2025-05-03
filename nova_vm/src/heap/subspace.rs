mod iso_subspace;
pub(crate) use iso_subspace::IsoSubspace;

use std::{
    any::type_name,
    ops::{Index, IndexMut},
};

use crate::engine::context::Bindable;

use super::HeapMarkAndSweep;

pub(crate) trait SubspaceKey: Sized + Copy + Eq + Ord {
    type HeapData: HeapMarkAndSweep + Bindable;
    const DEF: Self;
    fn get_index(self) -> usize;
    fn from_raw_index(index: usize) -> Self;

    #[inline]
    fn name() -> &'static str {
        type_name::<Self>()
    }
}

pub trait SubspaceStorage<K, D>
where
    D: ?Sized,
    Self: Index<K, Output = D> + IndexMut<K, Output = D>,
{
    /// Number of bytes allocated within this Subspace.
    ///
    /// Subspaces may choose to over-allocate for performance reasons, making its
    /// capacity larger than its size.
    fn size(&self) -> usize;
    // fn sweep(&mut self, gc: GcScope<'_, '_>);
    fn alloc(&mut self, value: D) -> K;
    // fn compact(&mut self, gc: GcScope<'_, '_>);

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

// TODO: full Subspace trait (SubspaceStorage + mark/sweep/compact methods)
