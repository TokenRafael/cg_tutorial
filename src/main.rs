#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;
extern crate gl;

mod util;
mod shader;
mod program;

use std::ffi::{CStr, CString};
use gl::types::*;

use shader::Shader;
use program::Program;

fn main() {
    // Initialize SDL
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Setup GL attributes
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    // Create a window
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl() // Setup window to receive GL context
        .resizable()
        .build()
        .unwrap();

    // Create an GL context
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Create a shader
    let vert_shader = Shader::from_source_vert(
        &CString::new(include_str!("../shaders/triangle.vert")).unwrap(),
    ).unwrap();

    let frag_shader = Shader::from_source_frag(
        &CString::new(include_str!("../shaders/triangle.frag")).unwrap(),
    ).unwrap();

    // Create a program
    let shader_program = Program::from_shadders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    // Create a vertex array object
    let vertices: Vec<f32> = vec![
        // Positions        // Colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,   // Bottom-left
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,   // Bottom-right
        0.0,  0.5, 0.0,     0.0, 0.0, 1.0,   // Top
    ];

    // Create bindings for shader rendering
    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    unsafe {
        // Create a vertex buffer object to host our vertices
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            util::sizeof(&vertices) as GLsizeiptr, // mem alloc (buffer size in bytes)
            vertices.as_ptr() as *const GLvoid, // void pointer to data
            gl::STATIC_DRAW, // usage
        );

        // Create a vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Setup vertex attributes
        gl::EnableVertexAttribArray(0); // layout (position = 0) in shader
        gl::VertexAttribPointer(
            0, // layout (position = 0) in shader
            3, // number of attributes
            gl::FLOAT, // type of data (f32)
            gl::FALSE, // normalized?
            (6 * std::mem::size_of::<GLfloat>()) as GLsizei, // stride (0 = tightly packed)
            std::ptr::null(), // offset of component
        );
        gl::EnableVertexAttribArray(1); // layout (position = 0) in shader
        gl::VertexAttribPointer(
            1, // layout (position = 0) in shader
            3, // number of attributes
            gl::FLOAT, // type of data (f32)
            gl::FALSE, // normalized?
            (6 * std::mem::size_of::<f32>()) as GLsizei, // stride (0 = tightly packed)
            (3 * std::mem::size_of::<f32>()) as *const GLvoid, // offset of component
        );

        // Bind cleanup
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        // Draw
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in loaded array
                3  // number of vertices to draw
            );
            gl::BindVertexArray(0);
        }

        // Swap window pixels (redraw)
        window.gl_swap_window();
    }
}
