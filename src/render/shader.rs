use gl;
use std;
use std::ffi::{CString, CStr};
use crate::resources::{Resources};
use glm;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: u32) -> Result<Shader, String> {
        let shader_id = shader_from_source(source, kind)?;
        Ok(Shader { id: shader_id })
    }
    pub fn vert_shader_from_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    } 
    
    pub fn frag_shader_from_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    } 

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, String> {
        
        // TODO: this should not panic, and instead print to console or debug log
        println!("about to do shader {}", name);

        let shader_type = super::EXTENSIONS.iter()
        .find(|&&x|
            name.ends_with(x.0)
        ).map(|&x| x.1).expect(&format!("Couldn't determine shader type for {}",name));
        let source = res.load_cstring(name).expect("Error loading shader");
        println!("returned shader {}", source.to_string_lossy());
        Shader::from_source(&source, shader_type)
    }

}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        // todo: review this, looks pretty dumb
        let error: CString = super::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
            );
        }
    
        return Err(error.to_string_lossy().into_owned());
    }
    
    Ok(id)
}