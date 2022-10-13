use core::cell::UnsafeCell;

pub mod interface {
    pub trait Mutex {
        type Data;
        fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
    }
}

pub struct NullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

// this should have + Send and + Sync
// it doesn't cause dono how to wrap the uart from the bsp layer
// sooo here we go. thus don't due anything skechy with it.
unsafe impl<T> Send for NullLock<T> where T: ?Sized {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }
}
