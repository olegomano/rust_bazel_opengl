extern crate material;
extern crate mesh;


#[derive(Clone)]
pub struct Geometry{
    mesh : mesh::Mesh,
    material : material::Material,
}
