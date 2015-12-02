#[macro_use]
extern crate gfx;


gfx_vertex!(_Foo {
    x@ _x: i8,
    y@ _y: f32,
    z@ _z: [u32; 4],
});

gfx_parameters!(_Bar {
    x@ _x: i32,
    y@ _y: [f32; 4],
    b@ _b: gfx::handle::RawBuffer<R>,
    t@ _t: gfx::shade::TextureParam<R>,
});

gfx_structure!(Vertex: _VertexDef {
    x@ _x: i8,
    y@ _y: f32,
    z@ _z: [u32; 4],
});

gfx_structure!(Instance: _InstanceDef {
    alpha@ _alpha: f32,
});

gfx_structure!(Local: _LocalDef {
    pos@ _pos: f32,
});

gfx_tex_format!(Rgba = gfx::tex::RGBA8);

gfx_pipeline_init!( _Data _Meta _Init = init {
    _vertex: gfx::VertexBuffer<Vertex> = gfx::PER_VERTEX,
    _instance: gfx::VertexBuffer<Instance> = gfx::PER_INSTANCE,
    _const_locals: gfx::ConstantBuffer<Local> = "Locals",
    _gobal: gfx::Constant<[f32; 4]> = "Global",
    //tex_diffuse: TextureView<Dim2, Float4>,
    //tex_normal: TextureView<Dim2, Float3>,
    //sampler_linear: Sampler,
    //buf_noise: BufferView<Int4>,
    //buf_frequency: UnorderedView<Dim2, Int>,
    pixel_color: gfx::RenderTarget<Rgba> = ("Color", gfx::state::MASK_ALL),
    //d@ depth: gfx::DepthStencilView<R>,
});
