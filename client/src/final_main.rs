extern crate ecs_proc_macro;
extern crate ecs;
extern crate app_context;
extern crate key_manager;
use std::any::TypeId;
use glam::Mat4;
use winit::keyboard::Key;

#[ecs_proc_macro::ecs(f32,Mat4)]
struct EcsSystem{}


impl app_context::App for EcsSystem{
    fn Init(&mut self){

    }

    fn Tick(&mut self){
        println!("I am Tick");
    }

    fn HandleKeyboard(&mut self, keyboard : &key_manager::KeyManager){ 
        if (keyboard.IsDown(Key::Character("w".into()))){
            println!("W is down!");
        }
    }
}



fn main(){
    let mut ecs = EcsSystem::new();
    let entity = ecs.entity_manager.NewEntity();
    
    let mut app_context = app_context::Context::new(&mut ecs);
    app_context.Run();
}
