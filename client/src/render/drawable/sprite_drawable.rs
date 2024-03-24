extern crate glam;
extern crate default_shader; 
extern crate vbo;
extern crate drawable;
use gl_context::gl;
extern crate gl_error;
use gl_error::CheckError;


pub struct SpriteManager{
    mesh : triangle_drawable::MeshDrawable,
    transform_list : std::vec::Vec<glam::Mat4>,
}


/*
 * Represents a collection of sprites that all share the same texture 
 */
impl SpriteManager{
    pub fn new(gl_context : &gl::Gl) -> Self{
        return Self{
            mesh : triangle_drawable::MeshDrawable::new_rect(gl_context),
            transform_list : std::vec::Vec::new(),
        }
    }
    
    pub fn AddSprite(&mut self, mat : &glam::Mat4) -> usize {
        self.transform_list.push(mat.clone());
        return self.transform_list.len() - 1;
    }

    pub fn UpdateSprite(&mut self, index : usize, mat : &glam::Mat4) {
        self.transform_list[index] = mat.clone();
    }

    pub fn GetSprite(&self, index : usize) -> &glam::Mat4 {
        return &self.transform_list[index];
    }

    pub fn Render(&self, 
            texture : &texture::Texture, 
            shader : &default_shader::DefaultShader,
            gl_context : &gl::Gl){

        self.mesh.SetTexture(texture); 
        texture.Bind();

        for transform in &self.transform_list {
            self.mesh.UpdateTransform(transform);
            shader.Render(&self.mesh,gl_context);
        }
    }    
} 
