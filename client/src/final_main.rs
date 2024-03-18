extern crate ecs_proc_macro;
extern crate ecs;
extern crate app_context;
use std::any::TypeId;
use glam::Mat4;

#[ecs_proc_macro::ecs(f32,Mat4)]
struct EcsSystem{}


impl app_context::App for EcsSystem{
    fn Tick(&mut self){
        println!("I am Tick");
    }
}



fn main(){
    let mut ecs = EcsSystem::new();
    let entity = ecs.entity_manager.NewEntity();
    
    let mut app_context = app_context::Context::new(&mut ecs);
    app_context.Run();
}
