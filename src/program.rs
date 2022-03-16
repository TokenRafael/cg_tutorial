use crate::{resources, Shader, util};
use gl::types::*;

/// Wrapper for OpenGL program
pub struct Program {
    gl: gl::Gl,
    id: GLuint,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource: {}", name)]
    ResourceLoad { name: String, inner: resources::Error },
    #[fail(display = "Cannot find shader type for {}", name)]
    UndefinedShaderType { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}

impl Program {
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    /// Creates a new program from shaders
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id()) };
        }
        unsafe { gl.LinkProgram(id) };

        // Error checking
        let mut success: GLint = 1;
        unsafe { gl.GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut len: GLint = 0;
            unsafe { gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) };

            let error = util::alloc_cstring_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar)
            };

            return Err(error.to_string_lossy().into_owned());
        }

        // Detach shaders so they can be deleted
        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id()) };
        }

        Ok(Program { gl: gl.clone(), id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) };
    }
}

#[macro_export]
macro_rules! shader_file {
    ($path:expr) => {
        cstr! {include_str!(concat!("../shaders/", $path))}
    };
}
