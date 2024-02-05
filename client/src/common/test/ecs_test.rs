extern crate ecs_proc_macro;
extern crate ecs;
use std::any::TypeId;

#[cfg(test)]
mod test {
    #[ecs_proc_macro::ecs(i32,f32)]
    struct EcsSystem{}


    #[test]
    fn ecs_test(){
        let ecs = EcsSystem::new();
        ecs.f32(&2.0,ecs::EntityId(0));
    }
}   
