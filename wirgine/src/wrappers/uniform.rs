use crate::{c_types::CUniform, c_functions::{wg_destroy_uniform, wg_uniform_set_current}};

use std::marker::PhantomData;

use crate::utils::IsReprC;

use libc::c_void;

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

    pub fn set_current(&self, data: &T) -> () {
        unsafe {
            wg_uniform_set_current(self.uniform, &*data as *const _ as *const c_void);
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
