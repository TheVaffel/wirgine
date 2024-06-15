use crate::{c_functions::{wg_cmd_start_recording, wg_cmd_bind_resource_set, wg_cmd_draw, wg_cmd_end_recording}, c_types::{CCommand, CVertexBuffer}};

use super::{framebuffer::Framebuffer, resource::ResourceBinding, vertex_buffer::{VertexBuffer, GenericVertexBuffer}, index_buffer::IndexBuffer};

pub struct Command {
    command: CCommand
}

impl Command {
    pub fn new(command: CCommand) -> Self {
        Self {
            command
        }
    }

    pub fn start_recording(&self, framebuffer: &Framebuffer) -> &Self {
        unsafe {
            wg_cmd_start_recording(self.command, framebuffer.get_framebuffer());
        }

        self
    }

    pub fn bind_resource_set(&self, set_index: u32, bindings: &Vec<ResourceBinding>) -> &Self {
        unsafe {
            wg_cmd_bind_resource_set(self.command, set_index, bindings.len() as u32, bindings[..].as_ptr());
        }

        self
    }

    pub fn draw(&self, vertex_buffers: &Vec<&dyn GenericVertexBuffer>, index_buffer: &IndexBuffer) -> &Self {
        let c_vertex_buffers: Vec<CVertexBuffer> = vertex_buffers.iter().map(|buffer| buffer.get_vertex_buffer()).collect();
        unsafe {
            wg_cmd_draw(self.command, c_vertex_buffers.len() as u32, c_vertex_buffers[..].as_ptr(), index_buffer.get_index_buffer());
        }

        self
    }

    pub fn end_recording(&self) -> () {
        unsafe {
            wg_cmd_end_recording(self.command);
        }
    }

    pub fn get_command(&self) -> CCommand {
        self.command
    }
}
