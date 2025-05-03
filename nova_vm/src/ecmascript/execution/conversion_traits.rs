use std::convert::Infallible;

use crate::engine::context::NoGcScope;

use super::Agent;

pub trait HeapFrom<'gc, T> {
    fn heap_from(agent: &mut Agent, value: T, gc: NoGcScope<'gc, '_>) -> Self;
}

pub trait HeapInto<'gc, T> {
    fn heap_into(agent: &mut Agent, value: Self, gc: NoGcScope<'gc, '_>) -> T;
}

pub trait TryHeapFrom<'gc, T>
where
    Self: Sized,
{
    type Error;
    fn try_heap_from(
        agent: &mut Agent,
        value: T,
        gc: NoGcScope<'gc, '_>,
    ) -> Result<Self, Self::Error>;
}

pub trait TryHeapInto<'gc, T>
where
    Self: Sized,
{
    type Error;
    fn try_heap_into(
        agent: &mut Agent,
        value: Self,
        gc: NoGcScope<'gc, '_>,
    ) -> Result<T, Self::Error>;
}

impl<'gc, T, U> HeapInto<'gc, U> for T
where
    U: HeapFrom<'gc, T>,
{
    #[inline]
    fn heap_into(agent: &mut Agent, value: Self, gc: NoGcScope<'gc, '_>) -> U {
        U::heap_from(agent, value, gc)
    }
}

impl<'gc, T, U> HeapFrom<'gc, T> for U
where
    U: From<T>,
{
    #[inline]
    fn heap_from(_agent: &mut Agent, value: T, _gc: NoGcScope<'gc, '_>) -> Self {
        U::from(value)
    }
}

impl<'gc, T, U> TryHeapInto<'gc, U> for T
where
    U: TryHeapFrom<'gc, T>,
{
    type Error = U::Error;

    #[inline]
    fn try_heap_into(
        agent: &mut Agent,
        value: Self,
        gc: NoGcScope<'gc, '_>,
    ) -> Result<U, Self::Error> {
        U::try_heap_from(agent, value, gc)
    }
}

impl<'gc, T, U> TryHeapFrom<'gc, U> for T
where
    U: HeapInto<'gc, T>,
{
    type Error = Infallible;

    #[inline]
    fn try_heap_from(
        agent: &mut Agent,
        value: U,
        gc: NoGcScope<'gc, '_>,
    ) -> Result<Self, Self::Error> {
        Ok(U::heap_into(agent, value, gc))
    }
}
