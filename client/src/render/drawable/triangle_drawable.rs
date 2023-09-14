extern crate glam;
extern crate shader; 
extern crate vbo;

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

pub struct TriangleDrawable {
    vbo : vbo::Vbo;
    pos_layout : PosLayout;
    uv_layout : UvLayout;
    transform : glam::Mat4;
}

pub impl TriangleDrawable {
    pub fn new(gl_context : &gl::Gl) -> Self{
        let vbo = vbo::Vbo::new();
        vbo.Load(gl_context,VERTEX_DATA);
        return {
            vbo : vbo,
            pos_layout : shader::Layout{
                count : 3.
                stride : 5,
            },
            uv_layout : shader::Layout{
                count : 2,
                stride : 5,
            },
            transform : glam::Mat4::Ident(),
        }
    }
    
    pub fn default() -> Self{
        return {
            vbo : vbo::Vbo::new(),
            pos_layout : PosLayout,
            uv_layout : UvLayout,
            transform : glam::Mat4::Ident(),
        }
    }

    pub fn SetTransform(&mut self, t : &glam::Mat4){
        self.transform = t;
    }
}

pub impl SptriteDrawable for TriangleDrawable{
    fn PosAttribute(&self) -> &shader::Layout {
        return &self.pos_layout;
    }

    fn UvAttribute(&self) -> &shader::Layout {
        return &self.uv_layout;
    }

    fn Buffer(&self) -> &vbo::Vbo {
        return &vbo;
    }

    fn Transform(&self) -> &glam::Mat4{
        return &transform;
    }
}
