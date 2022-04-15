/*   <--------Vertex Buffer-------->   */
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex
{
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        // defines how a buffer is represented in memory
        wgpu::VertexBufferLayout
        {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // how wide a vertex is
            step_mode: wgpu::VertexStepMode::Vertex,                            // tells the pipeline how often it should move to the next vertex
            attributes: &[
                wgpu::VertexAttribute                                           // Vertex attributes describe the individual parts of the vertex
                {
                    offset: 0,                                                  // bytes until the attribute starts
                    shader_location: 0,                                         // what location to store this attribute at
                    format: wgpu::VertexFormat::Float32x3,                      // shape of the attribute
                },
                wgpu::VertexAttribute
                {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
        // here the VertexBufferLayout is returned automatically
    }
}
 