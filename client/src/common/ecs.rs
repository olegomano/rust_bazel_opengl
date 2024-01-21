use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::UnsafeCell;

#[derive(Clone)]
pub struct EntityId(usize);
#[derive(Clone)]
pub struct ComponentId(usize);

#[derive(Clone)]
struct Component<T : Clone>{
    entity_id : EntityId,
    data : T
}

struct ComponentArray<T : Clone>{
    array : UnsafeCell<std::vec::Vec<Component<T>>>
}

#[derive(Clone)]
struct Entity{
    entity_id : EntityId,
    components : HashMap<TypeId,ComponentId>
}

/*
 * COMPONENT_ARRAY_TYPE is an enum of only ComponentArray specializations
 * ComponentId is globaly unique across all component types
 */
pub struct EntityManager<COMPONENT_ARRAY_TYPE>{
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
    entity_id : EntityId,
    component_id : ComponentId,
}

impl<COMPONENT_ARRAY_TYPE> EntityManager<COMPONENT_ARRAY_TYPE>{
    
    
    pub fn GenEntityId(&self) -> EntityId {
        return EntityId(0);
    }

    pub fn GenComponentId(&self) -> ComponentId {
        return ComponentId(0);
    }
}
