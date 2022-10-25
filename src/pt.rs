use std::{ 
    {marker::PhantomData},
    {cell::UnsafeCell},
    {ops::{Deref, DerefMut}},
    {ptr::NonNull}
};

#[derive(Debug, PartialEq, Clone)]
pub struct Ref<'a, T> {
    target: NonNull<Option<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Default> Deref for Ref<'a, T> {
    type Target = Option<T>;
    fn deref(&self) -> & Self::Target {
        unsafe { self.target.as_ref() }
    }
}

pub struct RefMut<'a, T> {
    target: NonNull<Option<T>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: Default> Deref for RefMut<'a, T> {
    type Target = Option<T>;
    fn deref(&self) -> & Self::Target {
        unsafe { self.target.as_ref() }
    }
}

impl<'a, T: Default> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.target.as_mut() }
    }
}

#[derive(Default, Debug)]
pub struct ParamType<T: Default>(UnsafeCell<Option<T>>);

impl<T: Default> ParamType<T> {
    fn new(t: T) -> Self {
        Self(
            UnsafeCell::new(Some(t))
        )
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.0.get()) };
        Ref{ 
            target: value,
            _marker: PhantomData,
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.0.get()) };
        RefMut{ 
            target: value,
            _marker: PhantomData,
        }
    }
}

impl<T: Default> Deref for ParamType<T> {
    type Target = Option<T>;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe { &*self.0.get() }
    }
}

impl<T: Default> DerefMut for ParamType<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        self.0.get_mut()
    }
}

impl<T: Default> PartialEq for  ParamType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T: Default + Clone> Clone for  ParamType<T> {
    fn clone(&self) -> Self {
        ParamType::<T>::new(unsafe { (self.0.get()).as_ref() }.unwrap().clone().expect("no value"))
    }
}