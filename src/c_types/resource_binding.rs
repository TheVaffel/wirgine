use libc::c_void;

#[repr(C)]
pub struct CResourceBinding {
    pub binding: u32,
    pub resource: *mut c_void
}
