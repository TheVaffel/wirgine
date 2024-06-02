use crate::{c_types::CUniform, c_functions::wg_destroy_uniform};

use std::marker::PhantomData;

use crate::utils::IsReprC;

pub struct Uniform<T: IsReprC> {
    uniform: CUniform,
    _data: PhantomData<T>
}

impl<T: IsReprC> Uniform<T> {
    pub fn new(uniform: CUniform) -> Self {
        Uniform::<T> {
            uniform,
            _data: PhantomData
        }
    }

    pub fn get_uniform(&self) -> CUniform {
        self.uniform
    }
}


impl<T: IsReprC> Drop for Uniform<T> {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_uniform(self.get_uniform());
        }
    }
}
