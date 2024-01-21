extern crate ecs_proc_macro;
extern crate ecs;

#[cfg(test)]
mod test {
    #[ecs_proc_macro::ecs(i32,f32)]
    struct EcsSystem{
        dummy_field : i32,
    }


    #[test]
    fn ecs_test(){
        let test_value = EcsSystem{
            dummy_field : 0,
        };
    }
}
