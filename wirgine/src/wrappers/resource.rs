use crate::c_types::CResourceBinding;

use libc::c_void;

pub type ResourceBinding = CResourceBinding;

pub trait Resource {
    fn get_ptr(&self) -> *mut c_void;
}

impl ResourceBinding {
    pub fn new<ResourceType: Resource>(binding: u32, resource: &ResourceType) -> Self {
        Self {
            binding,
            resource: resource.get_ptr(),
        }
    }
}
