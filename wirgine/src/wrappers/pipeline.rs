use crate::{c_functions::wg_destroy_pipeline, c_types::CPipeline};

use super::{shader::Shader, vertex_attrib_desc::VertexAttribDesc};

pub struct Pipeline {
    pipeline: CPipeline,
}

impl Pipeline {
    pub fn new(pipeline: CPipeline) -> Self {
        Pipeline { pipeline }
    }

    pub fn get_pipeline(&self) -> CPipeline {
        self.pipeline
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) -> () {
        unsafe {
            wg_destroy_pipeline(self.pipeline);
        }
    }
}
