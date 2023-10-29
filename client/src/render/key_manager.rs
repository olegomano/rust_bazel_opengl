use winit::event::{Event, WindowEvent,ElementState};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use winit::event::WindowEvent::KeyboardInput;
use winit::event::KeyEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::Key;

#[derive(Debug)]
pub struct KeyInfo{
    key : Key,
    down_time : Instant,
    hold_time : Duration,
    pressed : bool,
}

#[derive(Debug)]
pub struct KeyManager{
    key_map : HashMap<Key,KeyInfo>,
}

impl KeyManager{
    pub fn new() -> Self{
        return Self{
            key_map : HashMap::new(),
        }
    }
    
    pub fn KeyState(&self) -> &HashMap<Key,KeyInfo>{
        return &self.key_map;
    }
    
    pub fn IsDown(&self, key_code : Key) -> bool {
        if let Some(key_info) = self.key_map.get(&key_code){
            return key_info.pressed;
        }
        return false;
    }

    pub fn HandleInput(&mut self, event : KeyEvent ){
        let now = Instant::now();        
        match event.state {
            ElementState::Pressed => {
                self.key_map.insert(event.logical_key.clone(),KeyInfo{
                    key : event.logical_key.clone(),
                    down_time : now,
                    hold_time : Duration::default(),
                    pressed : true,
                });       
            }
        
            ElementState::Released => {
                if let Some(info) = self.key_map.get_mut(&event.logical_key) {
                    info.hold_time = now - info.down_time;
                    info.pressed = false;
                } 
            }
        }
    }   
}


