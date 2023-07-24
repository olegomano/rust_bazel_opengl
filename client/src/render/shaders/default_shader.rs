extern crate vbo;
extern crate shader;
extern crate gl_context;
use gl_context::gl;

const VERTEX_SHADER_SOURCE: &str = "
    #version 100
    precision mediump float;

    attribute vec2 position;
    attribute vec3 color;

    varying vec3 v_color;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_color = color;
    }";

const FRAGMENT_SHADER_SOURCE: &str= "
    #version 100
    precision mediump float;

    varying vec3 v_color;

    void main() {
        gl_FragColor = vec4(v_color, 1.0);
    }";


struct PosAttribute{}
impl shader::Attribute for PosAttribute{
    fn Count() -> gl::types::GLuint {
        return 3;
    }

    fn Stride() -> gl::types::GLuint {
        return 5;
    } 

    fn Name() -> &'static str {
        return "position";
    }
}

struct ColorAttribute{}
impl shader::Attribute for ColorAttribute{
    fn Count() -> gl::types::GLuint {
        return 2;
    }

    fn Stride() -> gl::types::GLuint {
        return 5;
    } 

    fn Name() -> &'static str {
        return "color";
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultShader{
    shader : shader::Shader,
    pos_attr : gl::types::GLint,
    uv_attr : gl::types::GLint,
}

impl DefaultShader{
    
    pub fn new(gl : &gl::Gl) -> Option<Self>{
        let s = shader::Shader::new(gl,VERTEX_SHADER_SOURCE,FRAGMENT_SHADER_SOURCE).expect("");
        return Some(Self{
            shader : s,
            pos_attr : s.BindAttrib::<PosAttribute>(gl),
            uv_attr :  s.BindAttrib::<ColorAttribute>(gl),
        })
    }
    
    pub fn default() -> Self{
        return Self{
            shader : shader::Shader::default(),
            pos_attr : 0,
            uv_attr : 0,
        }
    }

    /*
     * Draws the specified VBO with this shader
     * Assumes the VBO has a very specific layout
     */
    pub fn Render(&self, vbo : &vbo::Vbo, gl : &gl::Gl){
        vbo.Bind(gl);
        self.shader.SetAttrib::<PosAttribute>(self.pos_attr,gl);
        self.shader.SetAttrib::<ColorAttribute>(self.uv_attr,gl);
        unsafe{
            gl.DrawArrays(gl::TRIANGLES, 0, vbo.Elements());
        }
    }
}

