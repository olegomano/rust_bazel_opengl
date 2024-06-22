extern crate vbo;
extern crate shader;
extern crate gl_context;
extern crate drawable;
extern crate gl_utils;
extern crate glam;
extern crate gl_error;
use gl_error::CheckError;

use gl_context::gl;


const VERTEX_SHADER_SOURCE: &str = "
    #version 300 es
    precision mediump float;
    
    uniform mat4 uModelMatrix;
    uniform mat4 uCameraMatrix;

    in vec4 aPosition;
    in vec2 aUv;

    out vec2 v_color;

    void main() {
        gl_Position = uCameraMatrix * uModelMatrix * aPosition;        
        gl_Position.w = gl_Position.z + 1.0;
        gl_Position.z = gl_Position.z/ 1000.0 - 1.0;

        v_color = aUv;
    }
    \0";

const FRAGMENT_SHADER_SOURCE: &str= "
    #version 300 es
    precision mediump float;
    uniform sampler2D texture;

    in vec2 v_color;
    out vec4 FragColor;
    void main() {
        vec4 color = texture2D(texture, v_color.xy);
        FragColor = color;
    }
    \0";


pub struct BasicShader{
    shader : shader::Shader,
    u_transform : gl::types::GLint,
    u_camera : gl::types::GLint,
    a_pos : gl::types::GLint,
    a_uv : gl::types::GLint,
}

impl BasicShader{
    pub fn new(gl_contect : &gl::Gl) -> Self{    
        let s = shader::Shader::new(gl_context,VERTEX_SHADER_SOURCE,FRAGMENT_SHADER_SOURCE).expect("");
        s.Use(gl_context);
        return Self{
            shader : s,
            u_transform : s.BindUniformName(gl_context,"uModelMatrix"),
            u_camera : s.BindUniformName(gl_context,"uCameraMatrix"),
            a_pos : s.BindAttributeName(gl_context,"aPosition"),
            a_uv : s.BindAttributeName(gl_context,"aUv"),
        }

    }

    #[gl_utils::gl_error]
    pub fn Enable(&self,gl_context : &gl::Gl){
        unsafe{
            self.shader.Use(gl_context);
            gl_context.EnableVertexAttribArray(self.a_pos as gl::types::GLuint);
            gl_context.EnableVertexAttribArray(self.a_uv as gl::types::GLuint);

        }
    }
    
    pub fn Render(&self, gl_context : &gl::Gl){
        
    }
}
