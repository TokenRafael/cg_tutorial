use crate::resources::Resources;
use crate::{Program, Shader};
use gl::types::GLenum;
use crate::program::Error;

impl Shader {
    pub fn from_resources(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_ext, _)| {
                name.ends_with(file_ext)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::UndefinedShaderType { name: name.to_string() })?;

        let source = res.load(name)
            .map_err(|e| Error::ResourceLoad {
                name: name.to_string(),
                inner: e,
            })?;

        Shader::from_source(gl, &source, shader_kind)
    }
}

impl Program {
    pub fn from_resources(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let shaders = POSSIBLE_EXT.iter()
            .map(|&file_ext| {
                Shader::from_resources(gl, res, &format!("{}{}", name, file_ext))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Program::from_shaders(gl, &shaders).map_err(|e| Error::LinkError {
            name: name.to_string(),
            message: e,
        })
    }
}