use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::UnsafeCell;

extern crate glam;

pub struct Material{
    mass : f32
}

pub struct BoundingBox{
    width : f32,
    height : f32,
}

pub struct Component<T>{
    instance : usize,
    data : T
}

pub struct ComponentArray<T> {
    array : UnsafeCell<std::vec::Vec<Component<T>>>
} 

pub enum PhysicsComponent{
    Material(ComponentArray<Material>),
    BoundingBox(ComponentArray<BoundingBox>)
}

pub struct PhysicsItem{
    components : UnsafeCell<HashMap<TypeId,usize>>
}

pub struct PhysicsEngine{
    items : std::vec::Vec<PhysicsItem>,
    components : HashMap<TypeId,PhysicsComponent>,
}

impl PhysicsItem{
    pub fn new() -> Self{
        
    }
    
    pub fn AddComponent<T>(&self,handle : usize){
        let type_id = TypeId::of::<T>();
        unsafe{
            self.MutableComponents().insert(type_id,handle);
        }
    }
    
    pub fn HasComponent<T>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        return self.components.contains_key(type_id);
    }

    pub fn Component<T>(&self) -> &T{
        let type_id = TypeId::of::<T>();
        return self.components.get(type_id);
    }

    unsafe fn MutableComponents() -> &mut HashMap<TypeId,usize> {
        return &mut *self.components.get();
    }
}

impl<T> ComponentArray<T>{
    pub fn new() -> Self{

    }

    /*
     * Adds a new item to the array and returns its handle
     */
    pub fn Add(&self, instance : usize,data : &T) -> usize{
        unsafe{
             self.MutableArray().push(data);
        }
        return self.array.len();
    }

    unsafe fn MutableArray(&self) -> &mut std::vec::Vec<T> {
        return &mut *self.array.get();
    }
}

impl PhysicsEngine{
    pub fn new() -> Self{

    }

    pub fn Tick(&self) {

    }

    pub fn NewItem(&self) -> usize {
        let new_item = PhysicsItem{
            material : -1,
            bounding_box : -1,
        } 
        self.items.push(new_item);
        return self.items.len();
    }
    
    /*
     * Add component T to the specified instance instance 
     */
    pub fn AddComponent<T>(&self, instance : usize, component : &T) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(component_array) = self.components.get(type_id) {
            let handle = component_array.Add(instance,component)
            self.items[instance].AddComponent<T>(handle);
            return true;
        }else{
            println!("Failed to add componet {} to instance {}",type_id,instance);
            return false;
        }
    }
}
