
use core::marker;

#[doc = " Trait implemented by readable registers to enable the `read` method."]
pub trait Readable {}
pub trait Writeable {} 

pub struct Reg<U, REG> {
    register: vcell::VolatileCell<U>,
    _marker: marker::PhantomData<REG>,
}

impl<U, REG> Reg<U, REG>
where
    Self: Readable,
    U: Copy,
{
    pub fn read(&self) -> crate::R<U, Self> {
        crate::R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        }
    }
}

impl <U, REG> Reg<U, REG>
where
    Self: Readable + Writeable,
    U: Copy
{
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where for<'w> F: FnOnce(&R<U, Self>, &'w mut W<U, Self>) -> &'w mut W<U, Self>,
    {
        let bits = self.register.get();
        self.register.set(
            f(&R {
                bits,
                _reg: marker::PhantomData
            },
              &mut W {
                  bits,
                  _reg: marker::PhantomData
              }).bits,
        );
    }
}


pub struct R<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}

impl<U, T> R<U, T>
where
    U: Copy
{
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData
        }
    }

    pub fn bits(&self) -> U {
        self.bits
    }
}

pub struct W<U, REG> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<REG>,
}

impl<U, REG> W<U, REG> {
    pub unsafe fn bits(&mut self, bits: U) -> &mut Self {
        self.bits = bits;
        self
    }
}
