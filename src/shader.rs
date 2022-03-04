#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::util::alloc_cstring_len;
use gl::types::*;
use gl::Gl;
use std::ffi::CStr;
use crate::program::Error;
use crate::resources;

/// Interafaces with OpenGL Shaders and loads
/// shader source code from files.
/// # Arguments
/// * `source` - The file containing the shader source code.
/// * `kind` - The type of shader to load.
/// # Returns
/// The shader object.
/// # Errors
/// Returns an error if the shader could not be loaded.
fn shader_from_source(gl: &gl::Gl, source: &CStr, kind: GLenum) -> Result<GLuint, Error> {
    // Load shader
    let id = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    // Check if shader was created successfully
    let mut success: GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // Erro handling
    if success == 0 {
        let mut len: GLint = 0; // Erro buffer length
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = alloc_cstring_len(len as usize);

        // Get error
        unsafe { gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar) }
        return Err(Error::CompileError {
            name: String::from(source.to_str().unwrap()),
            message: String::from(error.to_str().unwrap()),
        });
    }

    Ok(id)
}

/// Shader wrapper for easy management
pub struct Shader {
    gl: gl::Gl,
    id: GLuint,
}

impl Shader {
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// Create a new shader from source of kind
    /// # Arguments
    /// * `source` - The file containing the shader source code.
    /// * `kind` - The type of shader to load.
    /// # Returns
    /// The shader object.
    /// # Errors
    /// Returns an error if the shader could not be loaded.
    pub fn from_source(gl: &gl::Gl, source: &CStr, kind: GLenum) -> Result<Self, Error> {
        let id = shader_from_source(&gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    /// Create a new shader from source of kind VERTEX_SHADER
    /// # Arguments
    /// * `source` - The file containing the shader source code.
    /// # Returns
    /// The shader object.
    /// # Errors
    /// Returns an error if the shader could not be loaded.
    pub fn from_source_vert(gl: &gl::Gl, source: &CStr) -> Result<Self, Error> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    /// Create a new shader from source of kind FRAGMENT_SHADER
    /// # Arguments
    /// * `source` - The file containing the shader source code.
    /// # Returns
    /// The shader object.
    /// # Errors
    /// Returns an error if the shader could not be loaded.
    pub fn from_source_frag(gl: &gl::Gl, source: &CStr) -> Result<Self, Error> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}
