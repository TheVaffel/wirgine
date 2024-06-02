
#[repr(C)]
#[derive(Clone)]
pub enum CComponentType {
    Float32 = 0,
    Float64 = 1,
    Int32 = 2,
    Int64 = 3
}

#[repr(C)]
#[derive(Clone)]
pub struct CVertexAttribDesc {
    pub binding_num: u32,
    pub component_type: CComponentType,
    pub num_components: u32,
    pub stride_in_bytes: u32,
    pub offset_in_bytes: u32,
    pub per_instance: u8
}


// Unused for now
#[link(name = "wingine_c")]
extern "C" {
    fn wg_create_attrib_desc(binding: u32,
                             comonent_type: CComponentType,
                             num_components: u32,
                             stride_in_bytes: u32,
                             offset_in_bytes: u32);
}
