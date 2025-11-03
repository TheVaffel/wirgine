#[repr(C)]
pub struct CCommandRenderPassSettings {
    pub num_color_attachments: u32,
    pub num_depth_attachments: u32,
    pub should_clear_color: u8,
    pub should_clear_depth: u8,
    pub finalize_as_texture: u8,
    pub clear_depth: f32,
    pub clear_color: [f32; 4],
}

#[repr(C)]
pub struct CDrawPassSettings {
    pub render_pass_settings: CCommandRenderPassSettings,
}

impl CDrawPassSettings {
    pub fn default() -> Self {
        CDrawPassSettings {
            render_pass_settings: CCommandRenderPassSettings {
                num_color_attachments: 1,
                num_depth_attachments: 1,
                should_clear_color: 0,
                should_clear_depth: 0,
                finalize_as_texture: 0,
                clear_depth: 1.0,
                clear_color: [0.8, 0.8, 0.8, 1.0],
            },
        }
    }
}
