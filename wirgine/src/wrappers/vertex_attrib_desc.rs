use crate::c_types::{CComponentType, CVertexAttribDesc};

pub type ComponentType = CComponentType;

impl ComponentType {
    fn byte_size(&self) -> usize {
        match self {
            CComponentType::Float32 => 4,
            CComponentType::Float64 => 8,
            CComponentType::Int32 => 4,
            CComponentType::Int64 => 8,
            _ => panic!("Unrecognized component type"),
        }
    }
}

pub struct VertexAttribDesc {
    attrib_desc: CVertexAttribDesc,
}

impl VertexAttribDesc {
    pub fn new(
        binding_num: u32,
        component_type: CComponentType,
        num_components: u32,
        stride_in_bytes: u32,
        offset_in_bytes: u32,
    ) -> Self {
        let actual_stride = if stride_in_bytes == 0 {
            component_type.byte_size() as u32 * num_components
        } else {
            stride_in_bytes
        } as u32;
        Self {
            attrib_desc: CVertexAttribDesc {
                binding_num,
                component_type,
                num_components,
                stride_in_bytes: actual_stride,
                offset_in_bytes,
                per_instance: 0,
            },
        }
    }

    pub fn new_instanced(
        binding_num: u32,
        component_type: CComponentType,
        num_components: u32,
        stride_in_bytes: u32,
        offset_in_bytes: u32,
    ) -> Self {
        let actual_stride = if stride_in_bytes == 0 {
            component_type.byte_size() as u32 * num_components
        } else {
            stride_in_bytes
        } as u32;
        Self {
            attrib_desc: CVertexAttribDesc {
                binding_num,
                component_type,
                num_components,
                stride_in_bytes: actual_stride,
                offset_in_bytes,
                per_instance: 1,
            },
        }
    }

    pub fn get_attrib_desc(&self) -> CVertexAttribDesc {
        self.attrib_desc.clone()
    }
}
