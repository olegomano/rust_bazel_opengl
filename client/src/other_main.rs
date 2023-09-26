extern crate window; 
extern crate default_shader; 
extern crate vbo; 
extern crate gl_context;
extern crate glam;
extern crate key_manager;
extern crate winit;
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

    fn HandleKeyboard(&self, keys : &key_manager::KeyManager, gl : &gl::Gl){
        if (keys.IsDown(winit::event::VirtualKeyCode::W)){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(0.05,0.0,0.0));
            self.tris.UpdateTransform(&(translate * self.tris.Transform()));
        }
        if (keys.IsDown(winit::event::VirtualKeyCode::S)){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(-0.05,0.0,0.0));
            self.tris.UpdateTransform(&(translate * self.tris.Transform()));
        }
    }
}

fn main(){
    let mut app = Box::new(window::AppInstance::<Data>::new());
    app.Run();
}
