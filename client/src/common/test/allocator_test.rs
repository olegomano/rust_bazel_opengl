extern crate allocator;

#[cfg(test)]
mod test {
    #[derive(Copy,Clone)]
    struct Holder{
        field : i32,
        field_1 : i32,
    }

    #[test]
    fn allocator_test(){
        println!("Started");
        let alloc = allocator::Allocator::<Holder>::new(63);
        let ptr_a = alloc.NewInit(&Holder{
            field : 32,
            field_1 : 33,
        });
        *ptr_a.GetMut() = Holder{
            field : 100,
            field_1 : 300,
        };
        for i in 1..=65 {
            let ptr = alloc.New().unwrap(); 
            alloc.Free(ptr);
        }
        
        assert_eq!(ptr_a.field,100);
        println!("Finished");
    }
}   
