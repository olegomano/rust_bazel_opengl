extern crate glam;
extern crate shader; 
extern crate vbo;
extern crate texture; 

pub trait SpriteDrawable{
    fn Transform(&self) -> glam::Mat4;
    fn PosAttribute(&self) -> &shader::Layout;
    fn UvAttribute(&self) -> &shader::Layout;
    fn Buffer(&self) -> &vbo::Vbo;
    fn Texture(&self) -> &texture::Texture;
}

