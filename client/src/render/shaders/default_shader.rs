extern crate vbo;
extern crate shader;
extern crate gl_context;
extern crate drawable;
extern crate gl_utils;
extern crate glam;

use gl_context::gl;

const VERTEX_SHADER_SOURCE: &str = "
    #version 300 es
    precision mediump float;
    
    uniform mat4 transform;
    
    in vec3 position;
    in vec2 color;

    out vec2 v_color;

    void main() {
        gl_Position = transform * vec4(position.x,position.y, 0.0, 1.0);        
        v_color = color;
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



unsafe fn CheckError(gl_context : &gl::Gl) -> Option<String>{
    let err =  gl_context.GetError();
    match err {
        gl::NO_ERROR => {
            return None;
        } 
        _ => {
            return Some(format!(
                "Gl Error:{}",err
            ));    
        }
    }
}



struct PosAttribute{}
impl shader::Attribute for PosAttribute{
    fn Name() -> &'static str {
        return "position\0";
    }
}

struct ColorAttribute{}
impl shader::Attribute for ColorAttribute{
    fn Name() -> &'static str {
        return "color\0";
    }
}

struct TransformUniform{}
impl shader::Attribute for TransformUniform{
    fn Name() -> &'static str {
        return "transform\0";
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultShader{
    shader : shader::Shader,
    transform_uniform : gl::types::GLint,
    pos_attr : gl::types::GLint,
    uv_attr : gl::types::GLint,
}



impl DefaultShader{
    #[gl_utils::gl_error]
    pub fn new(gl_context : &gl::Gl) -> Option<Self>{
        let s = shader::Shader::new(gl_context,VERTEX_SHADER_SOURCE,FRAGMENT_SHADER_SOURCE).expect("");
        s.Use(gl_context);
        return Some(Self{
            shader : s,
            pos_attr : s.BindAttrib::<PosAttribute>(gl_context),
            uv_attr :  s.BindAttrib::<ColorAttribute>(gl_context),
            transform_uniform : s.BindUniform::<TransformUniform>(gl_context),
        })
    }
    
    pub fn default() -> Self{
        return Self{
            shader : shader::Shader::default(),
            pos_attr : 0,
            uv_attr : 0,
            transform_uniform : 0,
        }
    }

    /*
     * Draws the specified VBO with this shader
     * Assumes the VBO has a very specific layout
     */
    pub fn Render(&self, drawable : &dyn drawable::SpriteDrawable, gl : &gl::Gl){
        self.Enable(gl);
        drawable.Buffer().Bind(gl);
        drawable.Texture().Bind(gl);
        self.shader.SetUniformMat(self.transform_uniform,&drawable.Transform(),gl);
        self.shader.SetAttrib(self.pos_attr,drawable.PosAttribute(),gl); 
        self.shader.SetAttrib(self.uv_attr,drawable.UvAttribute(),gl);
        unsafe{
            gl.DrawArrays(gl::TRIANGLES, 0, drawable.Buffer().Elements());
        }
    }

    #[gl_utils::gl_error]
    pub fn Enable(&self,gl_context : &gl::Gl){
        unsafe{
            self.shader.Use(gl_context);
            gl_context.EnableVertexAttribArray(self.pos_attr as gl::types::GLuint);
            gl_context.EnableVertexAttribArray(self.uv_attr as gl::types::GLuint);

        }
    }
}

