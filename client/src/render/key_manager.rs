use winit::event::{Event, WindowEvent,ElementState,VirtualKeyCode};
use winit::event::KeyboardInput;
use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct KeyInfo{
    key_code : VirtualKeyCode,
    down_time : Instant,
    hold_time : Duration,
    pressed : bool,
}

#[derive(Debug)]
pub struct KeyManager{
    key_map : HashMap<VirtualKeyCode,KeyInfo>,
}

impl KeyManager{
    pub fn new() -> Self{
        return Self{
            key_map : HashMap::new(),
        }
    }
    
    pub fn KeyState(&self) -> &HashMap<VirtualKeyCode,KeyInfo>{
        return &self.key_map;
    }
    
    pub fn IsDown(&self, key_code : VirtualKeyCode) -> bool {
        if let Some(key_info) = self.key_map.get(&key_code){
            return key_info.pressed;
        }
        return false;
    }

    pub fn HandleInput(&mut self, event : KeyboardInput){
        if let Some(key_code) = event.virtual_keycode {
            let now = Instant::now();
            
            match event.state {
                ElementState::Pressed => {
                    self.key_map.insert(key_code,KeyInfo{
                        key_code : key_code,
                        down_time : now,
                        hold_time : Duration::default(),
                        pressed : true,
                    });       
                }
                ElementState::Released => {
                    if let Some(info) = self.key_map.get_mut(&key_code) {
                        info.hold_time = now - info.down_time;
                        info.pressed = false;
                    } 
                }      
            }
        }
    }
}
