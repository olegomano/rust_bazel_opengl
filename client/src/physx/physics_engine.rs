extern crate glam;

pub struct Material{
    mass : f32
}

pub struct BoundingBox{
    width : f32,
    height : f32,
}

pub struct PhysicsEngine{
    bounding_boxes : std::vec::Vec<BoundingBox>,
}


impl PhysicsEngine{
    
    pub fn Tick() {

    }
}
