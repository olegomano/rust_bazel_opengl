extern crate window; 
use std::rc::{Rc, Weak};

struct Data{
    counter : i32
}

impl window::AppData for Data{
    fn new() -> Rc<Self>{
        return Rc::new(Data{
            counter : 0
        });
    }
    
    fn Draw(&self){
        println!("Draw");
    }

    fn Init(&self){
        println!("Init");
    }
}




fn main(){
    let app = Rc::new(window::AppInstance::<Data>::new());
    app.Run();
}
