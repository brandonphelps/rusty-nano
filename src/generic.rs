use core::marker;

#[doc = " Trait implemented by readable registers to enable the `read` method."]
pub trait Readable {}
pub trait Writeable {}

pub trait ResetValue {
    type Type;

    fn reset_value() -> Self::Type;
}

pub struct Reg<U, REG> {
    register: vcell::VolatileCell<U>,
    _marker: marker::PhantomData<REG>,
}

unsafe impl<U: Send, REG> Send for Reg<U, REG> {}

impl<U, REG> Reg<U, REG>
where
    U: Copy,
{
    pub fn as_ptr(&self) -> *mut U {
        self.register.as_ptr()
    }
}

impl<U, REG> Reg<U, REG>
where
    Self: Readable,
    U: Copy,
{
    #[inline(always)]
    pub fn read(&self) -> crate::R<U, Self> {
        crate::R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        }
    }
}

impl<U, REG> Reg<U, REG>
where
    Self: ResetValue<Type = U> + Writeable,
    U: Copy,
{
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
                _reg: marker::PhantomData,
            })
            .bits,
        );
    }
}

impl<U, REG> Reg<U, REG>
where
    Self: Readable + Writeable,
    U: Copy,
{
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R<U, Self>, &'w mut W<U, Self>) -> &'w mut W<U, Self>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &R {
                    bits,
                    _reg: marker::PhantomData,
                },
                &mut W {
                    bits,
                    _reg: marker::PhantomData,
                },
            )
            .bits,
        );
    }
}

pub struct R<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}

impl<U, T> R<U, T>
where
    U: Copy,
{
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }

    pub fn bits(&self) -> U {
        self.bits
    }
}

impl<FI> R<bool, FI> {
    pub fn bit(&self) -> bool {
        self.bits
    }
}

impl<U, T, FI> PartialEq<FI> for R<U, T>
where
    U: PartialEq,
    FI: Copy + Into<U>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&(*other).into())
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




#[doc = "Used if enumerated values cover not the whole range."]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant<U, T> {
    #[doc = " Expected variant."]
    Val(T),
    #[doc = " Raw bits."]
    Res(U),
}
