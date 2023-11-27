extern crate texture; 
extern crate window; 
extern crate default_shader; 
extern crate mesh_shader; 
extern crate vbo; 
extern crate gl_context;
extern crate glam;
extern crate key_manager;
extern crate winit;
extern crate asset_rs;
extern crate sprite_manager;
use drawable::SpriteDrawable;
use gl_context::gl;
use winit::keyboard::Key;
use std::rc::{Rc, Weak};
use std::mem::{self, MaybeUninit};

/*
 * Data only valid after the gl_context is initialized
 */
struct GlData{
    camera : glam::Mat4,
    shader : default_shader::DefaultShader,
    mesh_shader : mesh_shader::MeshShader,
    tris :  triangle_drawable::MeshDrawable,
    cube :  triangle_drawable::MeshDrawable,
    texture : texture::Texture,
    enemy_texture : texture::Texture,
    sprite_manager : sprite_manager::SpriteManager,
}

struct Data{
    counter : i32,
    gl_data : Option<GlData>,
}


impl Data{ 
    fn GenerateEnemySprites(count : i32, sprite_manager : &mut sprite_manager::SpriteManager){
        let scale = 0.25;
        let scale_matrix =  glam::Mat4::from_scale(glam::Vec3::new(scale,scale,scale));
        let theta = (360.0 / count as f64).to_radians();
        

        for i in 0..count {
            let x = (f64::sin(theta * i as f64)) as f32;
            let y = (f64::cos(theta * i as f64)) as f32;
            let transform = glam::Mat4::from_translation(glam::Vec3::new(x, y, 0.0)) * scale_matrix;
            
            sprite_manager.AddSprite(&transform);
        }
    }
}


impl window::AppData for Data{
    fn new() -> Box<Self>{
        return Box::new(Data{
            counter : 0,
            gl_data : None,
        });
    }

    fn Init(&mut self,gl : &gl::Gl){
        let shader = default_shader::DefaultShader::new(gl).unwrap();
        let mesh_shader = mesh_shader::MeshShader::new(gl).unwrap();

        let tris = triangle_drawable::MeshDrawable::new_rect(gl);
        let cube = triangle_drawable::MeshDrawable::new_cube(gl);
        let enemy_texture = texture::Texture::from_asset(&asset_rs::_image_Texture_brick_png::_asset,gl);
        let texture = texture::Texture::new(asset_rs::_image_Texture_brick_png::_asset.data,gl);
        tris.SetTexture(&texture);
        cube.SetTexture(&texture);
        
        let mut sprite_manager = sprite_manager::SpriteManager::new(gl);
        Self::GenerateEnemySprites(12,&mut sprite_manager);

        self.gl_data = Some(
            GlData{
                camera : glam::Mat4::IDENTITY,
                shader : shader,
                mesh_shader : mesh_shader,
                tris : tris,
                cube : cube,
                texture : texture,
                enemy_texture : enemy_texture,
                sprite_manager : sprite_manager,
            }
        );
    }

    fn Draw(&mut self,gl : &gl::Gl){
        let gl_data = self.gl_data.as_ref().unwrap();
        let angle = 1.0 as f64;
        let rotation_mat = glam::Mat4::from_axis_angle(glam::Vec3::new(1.0,1.0,0.0) , angle.to_radians() as f32);
        gl_data.cube.UpdateTransform(&(gl_data.cube.Transform() * rotation_mat));
        gl_data.mesh_shader.Render(&gl_data.cube,&gl_data.camera.inverse(),gl);
    }

    fn HandleKeyboard(&mut self, keys : &key_manager::KeyManager, gl : &gl::Gl){
        let gl_data : &mut GlData = self.gl_data.as_mut().unwrap();
         
        if (keys.IsDown(Key::Character("w".into()))){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(0.0,0.0,0.05));
            gl_data.camera  = gl_data.camera * translate;
        }
        if (keys.IsDown(Key::Character("s".into()))){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(0.0,0.00,-0.05));
            gl_data.camera  = gl_data.camera * translate;
        }
        if (keys.IsDown(Key::Character("a".into()))){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(0.05,0.0,0.0));
            gl_data.camera  = gl_data.camera * translate;
        }
        if (keys.IsDown(Key::Character("d".into()))){
            let translate = glam::Mat4::from_translation(glam::Vec3::new(-0.05,0.0,0.0));
            gl_data.camera  = gl_data.camera * translate;
        }
    }
}

fn main(){
    let mut app = Box::new(window::AppInstance::<Data>::new());
    app.Run();
}
