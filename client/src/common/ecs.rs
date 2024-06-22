use std::any::{Any, TypeId};

use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq,Clone,Copy,Debug)]
pub struct EntityId(pub usize);
#[derive(Eq, Hash, PartialEq,Clone,Copy,Debug)]
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

pub struct ComponentIterator<'a,T : Clone> {
    parent : &'a ComponentArray<T>,
    index : usize,
}

impl<'a,T : Clone> Iterator for ComponentIterator<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            if self.index >= (*self.parent.array.get()).len() {
                return None;
            }
            self.index = self.index + 1;
            return Some(&(*self.parent.array.get())[self.index - 1].data);
        }
    }
}



#[derive(Clone,Debug)]
pub struct Entity{
    pub entity_id : EntityId,
    pub components : HashMap<TypeId,ComponentId>
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
        unsafe{
            let mut_array : &mut std::vec::Vec<Component<T>> = &mut *self.array.get();
            let entity = Component::<T>{
                entity_id : entity_id,
                data : value.clone(),
            };
            mut_array.push(entity);
            return mut_array.len() - 1;
        }
    }
    
    pub fn Get(&self,index : usize) -> Option<&T> {
        unsafe{
            let mut_array : &mut std::vec::Vec<Component<T>> = &mut *self.array.get();
            if index >= mut_array.len() {
                return None;
            }
            return Some(&mut_array[index].data);
        }
    }

    pub fn into_iter(&self) -> ComponentIterator<T> {
        return ComponentIterator{
            parent : self,
            index : 0,
        }
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
    
    pub fn Entities(&self) -> impl Iterator<Item = &Entity> {
        return self._EntitiesMut().values();
    }

    pub fn NewEntity(&self) -> EntityId {
        let entities = self._EntitiesMut();

        let entity_id = self.GenEntityId();
        let new_map = HashMap::new();

        let new_entity = Entity{
            entity_id : entity_id,
            components : new_map,
        };
        entities.insert(entity_id,new_entity);
        return entity_id;
    }

    pub fn GetEntity(&self, entity_id : &EntityId) -> Option<&Entity> {
        return self._EntitiesMut().get(entity_id);
    }

    /*
     * Regiters the component_id  into all the book keeping structures
     * 
     * This does not allocate the component. component_index must be a valid entry in the
     * ComponentArray
     */
    pub fn RegisterComponent(&self,entity_id : EntityId, type_id : &TypeId, component_index : usize) -> ComponentId {
        let component_id = self.GenComponentId();
        let component_type_map = self._ComponentTypeMapMut();
        let component_index_map = self._ComponentIndexMapMut();
        let entities_map = self._EntitiesMut();

        component_type_map.insert(component_id,type_id.clone());
        component_index_map.insert(component_id,component_index);
        
        let entity : &mut Entity = entities_map.get_mut(&entity_id).unwrap();
        entity.components.insert(type_id.clone(),component_id);
        return component_id;
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

    pub fn _EntitiesMut(&self) -> &mut HashMap<EntityId,Entity> {
        unsafe{
            return &mut *self.entities.get();
        }
    }

    pub fn _ComponentIndexMapMut(&self) -> &mut HashMap<ComponentId,usize> {
        unsafe{
            return &mut *self.component_index_map.get();
        }
    }

    pub fn _ComponentTypeMapMut(&self) -> &mut HashMap<ComponentId,TypeId> {
        unsafe{
            return &mut *self.component_type_map.get();
        }
    }
}

