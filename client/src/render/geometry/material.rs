extern crate texture;

#[derive(Clone)]
pub struct BasicMaterial{
    albedo : texture::Texture,    
    normal : texture::Texture,
}

#[derive(Clone)]
pub struct PbrMaterial{}

#[derive(Clone)]
pub enum Material{
    Basic(BasicMaterial),
    Pbr(PbrMaterial),
    Missing(),
}


impl Material{
    pub fn Albedo(&self) -> texture::Texture {
        match self {
            Material::Basic(basic) => {
                return basic.albedo.clone();
            }
            _ => {
                return texture::Texture::default();
            }

        }
    }


    pub fn Normal(&self) -> texture::Texture {
        match self {
            Material::Basic(basic) => {
                return basic.normal.clone();
            }
            _ => {
                return texture::Texture::default();
            }

        }
    }

}


impl BasicMaterial{
    pub fn new( a : texture::Texture, n : texture::Texture ) -> Self{
        return Self{
            albedo : a,
            normal : n,
        };
    }
}

impl PbrMaterial{
    pub fn new() -> Self{
        return Self{};
    }
}
