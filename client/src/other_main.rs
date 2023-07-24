extern crate window; 
extern crate default_shader; 
extern crate vbo; 
extern crate gl_context;
use gl_context::gl;

use std::rc::{Rc, Weak};

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


struct Data{
    counter : i32,
    tris : vbo::Vbo,
    shader : std::cell::Cell<default_shader::DefaultShader>,
}

impl window::AppData for Data{
    fn new() -> Rc<Self>{
        return Rc::new(Data{
            counter : 0,
            tris : vbo::Vbo::new(),
            shader : std::cell::Cell::new(  default_shader::DefaultShader::default() ), 
        });
    }
    
    fn Draw(&self,gl : &gl::Gl){
        self.shader.get().Render(&self.tris,gl);                
    }

    fn Init(&self,gl : &gl::Gl){
        println!("Init");
        self.tris.InitTriangle(gl);
        self.shader.replace( default_shader::DefaultShader::new(gl).unwrap() );
    }
}




fn main(){
    let mut app = Rc::new(window::AppInstance::<Data>::new());
    app.Run();
}
