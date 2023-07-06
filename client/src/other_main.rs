extern crate window; 
extern crate shader; 
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
    counter : i32
}

impl window::AppData for Data{
    fn new() -> Rc<Self>{
        return Rc::new(Data{
            counter : 0
        });
    }
    
    fn Draw(&self){
     
    }

    fn Init(&self,gl : &gl::Gl){
        println!("Init");
        let shader = shader::Shader::new(gl,VERTEX_SHADER_SOURCE,FRAGMENT_SHADER_SOURCE);
    }
}




fn main(){
    let app = Rc::new(window::AppInstance::<Data>::new());
    app.Run();
}
