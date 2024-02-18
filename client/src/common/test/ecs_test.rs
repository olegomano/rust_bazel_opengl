extern crate ecs_proc_macro;
extern crate ecs;
use std::any::TypeId;

#[cfg(test)]
mod test {
    #[ecs_proc_macro::ecs(i32,f32)]
    struct EcsSystem{}


    #[test]
    fn ecs_test(){
        print!("Started");
        let ecs = EcsSystem::new();
        let entity = ecs.entity_manager.NewEntity();
        ecs.Addf32(&2.0,entity);
        for float in ecs.Iteratef32() {
            println!("{}",float);
        }
        for entity in ecs.Entities() {
            println!("{:?}",entity);
        }
        print!("Finished");
    }
}   
