extern crate ecs_proc_macro;
extern crate ecs;
extern crate app_context;
extern crate key_manager;
extern crate geometry;
extern crate mesh;
extern crate material;
extern crate gl_context;
use gl_context::gl;
use std::cell::OnceCell;
use std::any::TypeId;
use glam::Mat4;
use winit::keyboard::Key;

#[ecs_proc_macro::ecs(f32,Mat4)]
struct EcsSystem{}


struct AppInstance{
    cube_mesh : OnceCell<mesh::Mesh>,
    ecs_system : EcsSystem,
}

impl AppInstance{
    pub fn new() -> Self{
        return Self{
            ecs_system : EcsSystem::new(),
            cube_mesh : OnceCell::new(),
        }   
    }
}

impl app_context::App for AppInstance{
    fn Init(&mut self,gl_context : &gl::Gl){
        let entity = self.ecs_system.entity_manager.NewEntity();
        self.cube_mesh.set(mesh::Mesh::NewCube(gl_context));
    }

    fn Tick(&mut self, gl_context : &gl::Gl){
        println!("I am Tick");
    }
    
    fn Render(&mut self, gl_context : &gl::Gl){
        println!("I am Render");
    }

    fn HandleKeyboard(&mut self, keyboard : &key_manager::KeyManager, gl_context : &gl::Gl){ 
        if (keyboard.IsDown(Key::Character("w".into()))){
            println!("W is down!");
        }
    }
}

fn main(){
    let mut instance = AppInstance::new();
    let mut app_context = app_context::Context::new(&mut instance);
    app_context.Run();
}
