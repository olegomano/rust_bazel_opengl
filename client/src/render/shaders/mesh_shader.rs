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
        return "aPosition\0";
    }
}

struct UvAttribute{}
impl shader::Attribute for UvAttribute{
    fn Name() -> &'static str {
        return "aUv\0";
    }
}

struct ModelMatrixUniform{}
impl shader::Attribute for ModelMatrixUniform{
    fn Name() -> &'static str {
        return "uModelMatrix\0";
    }
}

struct CameraMatrixUniform{}
impl shader::Attribute for CameraMatrixUniform{
    fn Name() -> &'static str {
        return "uCameraMatrix\0";
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct MeshShader{
    shader : shader::Shader,
    transform_uniform : gl::types::GLint,
    camera_uniform : gl::types::GLint,
    pos_attr : gl::types::GLint,
    uv_attr : gl::types::GLint,
}

impl MeshShader{
    pub fn new(gl_context : &gl::Gl) -> Option<Self>{
        let s = shader::Shader::new(gl_context,VERTEX_SHADER_SOURCE,FRAGMENT_SHADER_SOURCE).expect("");
        s.Use(gl_context);
        return Some(Self{
            shader : s,
            pos_attr : s.BindAttrib::<PosAttribute>(gl_context),
            uv_attr :  s.BindAttrib::<UvAttribute>(gl_context),
            transform_uniform : s.BindUniform::<ModelMatrixUniform>(gl_context),
            camera_uniform : s.BindUniform::<CameraMatrixUniform>(gl_context),
        })
    
    }

    #[gl_utils::gl_error]
    pub fn Enable(&self,gl_context : &gl::Gl){
        unsafe{
            self.shader.Use(gl_context);
            gl_context.EnableVertexAttribArray(self.pos_attr as gl::types::GLuint);
            gl_context.EnableVertexAttribArray(self.uv_attr as gl::types::GLuint);

        }
    }
    
    #[gl_utils::gl_error]
    pub fn Render(&self,drawable : &dyn drawable::SpriteDrawable, camera : &glam::Mat4, gl_context : &gl::Gl){
        self.Enable(gl_context);
        drawable.Buffer().Bind(gl_context);
        drawable.Texture().Bind(gl_context);
        self.shader.SetUniformMat(self.transform_uniform,&drawable.Transform(),gl_context);
        self.shader.SetUniformMat(self.camera_uniform,camera,gl_context);
        self.shader.SetAttrib(self.pos_attr,drawable.PosAttribute(),gl_context); 
        self.shader.SetAttrib(self.uv_attr,drawable.UvAttribute(),gl_context);
        unsafe{
            gl_context.DrawArrays(gl::TRIANGLES, 0, drawable.Buffer().Elements());
        }
    }
}



















