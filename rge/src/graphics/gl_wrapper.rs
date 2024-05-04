/* TODO - research the non-binding way to do everything and refactor the code to implement it*/

use std::{collections::HashMap, fs::File};
use std::io::Read;
use std::ffi::CString;
use std::{mem, ptr};
use std::os::raw::c_void;
use std::sync::Mutex;

use cgmath::Matrix;
use gl::types::{GLboolean, GLenum, GLfloat, GLsizeiptr, GLuint, GLint, GLsizei, GLchar};


pub struct Vao {
    id: GLuint,
}
impl Vao {
    pub fn new () -> Vao {
        let mut id = 0;
        unsafe {
            // Generates a single Vertex Array Object (VAO), reserves a spot for it
            // in OpenGL's internal data structures and gives us back an id that we
            // store in our struct for future use.
            gl::GenVertexArrays(1, &mut id)
        }

        Vao {id}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

}

pub struct BufferObject {
    id: GLuint,
    r#type: GLenum,
    usage: GLenum,
}
impl BufferObject {
    pub fn new(r#type: GLenum, usage: GLenum) -> BufferObject{
        let mut id: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        BufferObject {id, r#type, usage}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData (
                self.r#type,
                // Calculates the size of the buffer in bytes (you have to cast the type as GLsizeiptr),
                // because the default size_of function yeilds an unsigned 32-bit integer, and for some
                // reason Glsizeiptr is signed, https://stackoverflow.com/questions/8996743/why-isnt-glsizei-defined-as-unsigned)
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                // After storing the byte size, store the pointer of the array so we can now
                // access the entire array
                data.as_ptr() as *const c_void,
                self.usage,
            )
        }
    } 

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData (
                self.r#type,
                (data.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                self.usage,
            )
        }
    } 

    // Not sure if this is a good idea or not but fuck it
    pub fn store_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData (
                self.r#type,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                self.usage,
            )
        }
    } 

}

pub struct VertexAttribute {
    index: GLuint,
}
impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute{
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer)
        }
        VertexAttribute {index}
    }

    pub fn enable(&self){
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self){
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct ShaderProgram {
    program: u32,
    uniform_ids: HashMap<String, GLint>,
}


use lazy_static::lazy_static;

lazy_static! {
    pub static ref SHADER_PROGRAM: Mutex<Option<ShaderProgram>> = Mutex::new(None);
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {

    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path).unwrap_or_else(|_| panic!("Failed to open {:?}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path).unwrap_or_else(|_| panic!("Failed to open {:?}", fragment_shader_path));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file.read_to_string(&mut vertex_shader_source).expect("Failed to read vertex shader");
        fragment_shader_file.read_to_string(&mut fragment_shader_source).expect("Failed to read fragment shader");
    
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vrt = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vrt.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_vrt = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_vrt.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512); // Vec that will hold the log
            info_log.set_len(511); // Set length to exclude the trailing null character
            
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", 
                        std::str::from_utf8(&info_log).unwrap());
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            

            ShaderProgram {
                program,
                uniform_ids: HashMap::new(),
            }
        }

    }


    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn load_uniform (&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(uniform_name).unwrap().as_ptr())
        };
        if uniform_location < 0 { // OpenGL could not locate the uniform
            panic!("Cannot locate uniform {:}", uniform_name)
        } else {
            self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_float_uniform(&self, uniform_name: &str, value: f32){
        unsafe {
            gl::Uniform1f(
                self.uniform_ids[uniform_name],
                value,
            );
        }
    }

    pub fn set_vec3_uniform(&self, uniform_name: &str, vector: [f32; 3]) {
        unsafe {
            gl::Uniform3fv(
                self.uniform_ids[uniform_name],
                1,
                vector.as_ptr(),
            )
        }
    }

    // We'll need matrix4's for our vertex shader eventually 
    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }

}