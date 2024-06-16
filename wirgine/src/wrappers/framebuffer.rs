use crate::c_types::CFramebuffer;

pub struct Framebuffer {
    framebuffer: CFramebuffer,
}

impl Framebuffer {
    pub fn new(framebuffer: CFramebuffer) -> Self {
        Self { framebuffer }
    }

    pub fn get_framebuffer(&self) -> CFramebuffer {
        self.framebuffer
    }
}
