use std::any::{Any, TypeId};

use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::collections::HashMap;

#[derive(Clone,Copy,Debug)]
pub struct EntityId(pub usize);
#[derive(Clone,Copy,Debug)]
pub struct ComponentId(pub usize);

#[derive(Clone,Debug)]
struct Component<T : Clone>{
    entity_id : EntityId,
    data : T
}

#[derive(Debug)]
pub struct ComponentArray<T : Clone>{
    array : UnsafeCell<std::vec::Vec<Component<T>>>
}

#[derive(Clone,Debug)]
struct Entity{
    entity_id : EntityId,
    components : HashMap<TypeId,ComponentId>
}

/*
 * COMPONENT_ARRAY_TYPE is an enum of only ComponentArray specializations
 * ComponentId is globaly unique across all component types
 */
pub struct EntityManager<COMPONENT_TYPE,COMPONENT_ARRAY_TYPE>{
    components : UnsafeCell<HashMap<TypeId,COMPONENT_ARRAY_TYPE>>,
    entities : UnsafeCell<HashMap<EntityId,Entity>>,    
    /*
     * Maps from a ComponentId to the index in the component array
     */
    component_index_map : UnsafeCell<HashMap<ComponentId,usize>>,
    component_type_map : UnsafeCell<HashMap<ComponentId,TypeId>>,
    /*
     * For generating unique entity/component id
     */
    entity_id : UnsafeCell<EntityId>,
    component_id : UnsafeCell<ComponentId>,

    /*
     * Phantom
     */
    phantom_enum: PhantomData<COMPONENT_TYPE>,
}

impl<T : Clone> ComponentArray<T> {
    pub fn new() -> Self{
        return Self{
            array : UnsafeCell::new(std::vec::Vec::new()),
        }
    }

    /*
     * Allocates a new component with the specified entity_id
     */
    pub fn Allocate(&self, entity_id : EntityId, value : &T) -> usize {
        return 0; 
    }
}

impl<COMPONENT_TYPE,COMPONENT_ARRAY_TYPE> EntityManager<COMPONENT_TYPE,COMPONENT_ARRAY_TYPE>{
    
    pub fn new() -> Self{
        let components = HashMap::new();
        let entities = HashMap::new();
        let component_index_map = HashMap::new();
        let components_type_map = HashMap::new();

        return Self{
            components : UnsafeCell::new( components  ),
            entities : UnsafeCell::new( entities ),
            component_index_map : UnsafeCell::new( component_index_map ),
            component_type_map : UnsafeCell::new( components_type_map ),


            entity_id : UnsafeCell::new(EntityId(0)),
            component_id : UnsafeCell::new(ComponentId(0)),
            
            phantom_enum : PhantomData,
        }
    }

    pub fn NewEntity(&self) -> EntityId {
        let entity_id = self.GenEntityId();
        let new_map = HashMap::new();

        let new_entity = Entity{
            entity_id : entity_id,
            components : new_map,
        };
        //self.entities[entity_id] = new_entity;
        return entity_id;
    }

    /*
     * Regiters the component_id  into all the book keeping structures
     * 
     * This does not allocate the component. component_index must be a valid entry in the
     * ComponentArray
     */
    pub fn RegisterComponent(&self,entity : EntityId, type_id : &TypeId, component_index : usize) -> ComponentId {
        return ComponentId(0);
    }


    pub fn GenEntityId(&self) -> EntityId {
        unsafe{
            let curr_value = (*self.entity_id.get()).0 + 1;
            *self.entity_id.get() = EntityId(curr_value);
            return *self.entity_id.get();
        }
    }

    pub fn GenComponentId(&self) -> ComponentId {
        unsafe{
            let curr_value = (*self.component_id.get()).0 + 1;
            *self.component_id.get() = ComponentId(curr_value);
            return *self.component_id.get();
        }
    }
    
    pub fn Components(&self) -> &HashMap<TypeId,COMPONENT_ARRAY_TYPE> {
        unsafe { 
            return &*self.components.get(); 
        }
    }
    
    /*
     * For internal use only
     */
    pub fn _ComponentsMut(&self) -> &mut HashMap<TypeId,COMPONENT_ARRAY_TYPE> {
        unsafe { 
            return &mut *self.components.get(); 
        }
    }

}
