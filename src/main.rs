#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate gl;
extern crate ogl_main;
extern crate sdl2;

mod program;
mod shader;
mod util;
mod resources;
mod from_resource;

use gl::types::*;
use ogl_main::ogl_main;
use std::ffi::{CStr, CString};
use std::path::Path;

use program::Program;
use shader::Shader;

#[ogl_main(title = "Gamer", window = "800x600", bg_color = "0.3 0.3 0.5 1.0")]
fn main() {
    // Load resources
    let res = resources::Resources::from_rel_path(Path::new("shaders")).unwrap();

    // Create shader program from resources loaded
    let shader_program = Program::from_resources(&gl, &res, "triangle").unwrap();
    shader_program.set_used();

    // Create a vertex array object
    let vertices: Vec<f32> = vec![
        // Positions        // Colors
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // Bottom-left
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // Bottom-right
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // Top
    ];

    // Create bindings for shader rendering
    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    unsafe {
        // Create a vertex buffer object to host our vertices
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,                      // target
            util::sizeof(&vertices) as GLsizeiptr, // mem alloc (buffer size in bytes)
            vertices.as_ptr() as *const GLvoid,    // void pointer to data
            gl::STATIC_DRAW,                       // usage
        );

        // Create a vertex array object
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        // Setup vertex attributes
        gl.EnableVertexAttribArray(0); // layout (position = 0) in shader
        gl.VertexAttribPointer(
            0,                                               // layout (position = 0) in shader
            3,                                               // number of attributes
            gl::FLOAT,                                       // type of data (f32)
            gl::FALSE,                                       // normalized?
            (6 * std::mem::size_of::<GLfloat>()) as GLsizei, // stride (0 = tightly packed)
            std::ptr::null(),                                // offset of component
        );
        gl.EnableVertexAttribArray(1); // layout (position = 0) in shader
        gl.VertexAttribPointer(
            1,                                                 // layout (position = 0) in shader
            3,                                                 // number of attributes
            gl::FLOAT,                                         // type of data (f32)
            gl::FALSE,                                         // normalized?
            (6 * std::mem::size_of::<f32>()) as GLsizei,       // stride (0 = tightly packed)
            (3 * std::mem::size_of::<f32>()) as *const GLvoid, // offset of component
        );

        // Bind cleanup
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    // Handle user input
    let mut event_pump = sdl.event_pump().unwrap();
    'render: loop {
        for event in event_pump.poll_iter() {
            // Handle any event
            match event {
                sdl2::event::Event::Quit { .. } => break 'render,
                _ => {}
            }
        }

        // Render window contents
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in loaded array
                3,             // number of vertices to draw
            );
            gl.BindVertexArray(0);
        }

        // Swap window pixels (redraw)
        window.gl_swap_window();
    }
}
