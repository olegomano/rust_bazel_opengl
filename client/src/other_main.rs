extern crate window; 
extern crate default_shader; 
extern crate vbo; 
extern crate gl_context;
extern crate drawable;
use gl_context::gl;

use std::rc::{Rc, Weak};


struct Data{
    counter : i32,
    shader : std::cell::Cell<default_shader::DefaultShader>,
    tris :  std::cell::Cell<drawable::TriangleDrawable>,
}

impl window::AppData for Data{
    fn new() -> Rc<Self>{
        return Rc::new(Data{
            counter : 0,
            shader : std::cell::Cell::new(  default_shader::DefaultShader::default() ), 
            tris : std::cell::Cell::new( drawable::TriangleDrawable::default()),
        });
    }
     
    fn Draw(&self,gl : &gl::Gl){
        self.shader.get().Render(&self.tris,gl);                
    }

    fn Init(&self,gl : &gl::Gl){
        self.shader.replace( default_shader::DefaultShader::new(gl).unwrap() );
        self.tris.replace(drawable::TriangleDrawable::new(gl));
    }
}

fn main(){
    let mut app = Rc::new(window::AppInstance::<Data>::new());
    app.Run();
}
