extern crate ecs_proc_macro;
extern crate ecs;
extern crate app_context;
extern crate key_manager;
use std::any::TypeId;
use glam::Mat4;
use winit::keyboard::Key;

#[ecs_proc_macro::ecs(f32,Mat4)]
struct EcsSystem{}


struct AppInstance{
    ecs_system : EcsSystem,
}

impl AppInstance{
    pub fn new() -> Self{
        return Self{
            ecs_system : EcsSystem::new()
        }   
    }
}

impl app_context::App for AppInstance{
    fn Init(&mut self){
        let entity = self.ecs_system.entity_manager.NewEntity();
    }

    fn Tick(&mut self){
        println!("I am Tick");
    }
    
    fn Render(&mut self){
        println!("I am Render");
    }

    fn HandleKeyboard(&mut self, keyboard : &key_manager::KeyManager){ 
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
