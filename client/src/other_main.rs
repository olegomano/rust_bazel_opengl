extern crate window; 
extern crate default_shader; 
extern crate vbo; 
extern crate gl_context;
extern crate glam;
use drawable::SpriteDrawable;
use gl_context::gl;

use std::rc::{Rc, Weak};


struct Data{
    counter : i32,
    shader : std::cell::Cell<default_shader::DefaultShader>,
    tris :  triangle_drawable::TriangleDrawable,
}

impl window::AppData for Data{
    fn new() -> Rc<Self>{
        return Rc::new(Data{
            counter : 0,
            shader : std::cell::Cell::new(  default_shader::DefaultShader::default() ), 
            tris : triangle_drawable::TriangleDrawable::default(),
        });
    }
     
    fn Draw(&self,gl : &gl::Gl){
        let rotate = glam::Mat4::from_rotation_z(0.1);
        self.tris.UpdateTransform(&(rotate * self.tris.Transform()));
        self.shader.get().Render(&self.tris,gl);                
    }

    fn Init(&self,gl : &gl::Gl){
        self.shader.replace( default_shader::DefaultShader::new(gl).unwrap() );
        self.tris.Load(gl);
    }
}

fn main(){
    let mut app = Rc::new(window::AppInstance::<Data>::new());
    app.Run();
}
