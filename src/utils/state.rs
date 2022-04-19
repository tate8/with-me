use wgpu::util::DeviceExt;
use winit::
{
    event::*,
    window::{Window},
};

use super::
{
    vertex,
    texture,
};


/*
    we store all the unique vertices in VERTICES and 
    we create another buffer that stores indices to 
    elements in VERTICES to create the triangles
*/
// Changed
const VERTICES: &[vertex::Vertex] = 
&[
    vertex::Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], },
    vertex::Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], },
    vertex::Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], },
    vertex::Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], },
    vertex::Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], },
];
const INDICES: &[u16] = 
&[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];


/*   <--------Global State-------->   */
pub struct State
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
}

impl State
{
    // the new function creates a new global state
    // This is like the constructor
    pub async fn new(window: &Window) -> Self 
    {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Its main purpose is to create Adapters and Surfaces.
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // The surface is the part of the window that we draw to
        let surface = unsafe { instance.create_surface(window) };

        // The adapter is a handle to the actual graphics card
        // You can use it to get info about the card
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // Get device and queue from adapter
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        // This is the config for the surface
        // it will define how the surface creates its underlying SurfaceTextures
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,              // how SurfaceTextures are used
            format: surface.get_preferred_format(&adapter).unwrap(),    // how SurfaceTextures are stored on gpu
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,                      // how to sync surface with display
        };
        surface.configure(&device, &config);





        // Texture creation (see 'texture.rs')
        let diffuse_bytes = include_bytes!("../../images/happy_tree.png");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "../../images/happy_tree.png").unwrap();


        // A BindGroup describes a set of resources and how they can be accessed by a shader
        let texture_bind_group_layout = 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture
                        {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // a BindGroup is a more specific declaration of the BindGroupLayout
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        // <--------------END-------------->



        
        
        let clear_color = wgpu::Color::BLACK;

        // <--------------Making Render Pipeline-------------->

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor
        {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        // pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
        {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            // Vertex shader
            vertex: wgpu::VertexState
            {
                module: &shader,
                entry_point: "vs_main",             // entry point of shader (name of fn)
                buffers: &[vertex::Vertex::desc(),],        // what type of vertices we want to pass to the vertex shader
            },

            // Fragment shader
            fragment: Some(wgpu::FragmentState 
            {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState   // what color outputs it should set up
                {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),

            // The primitive field describes how to interpret our vertices when converting them into triangles
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,    // each three vertices will correspond to one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,                   // tell wgpu how to determine whether a given triangle is facing forward or not
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                                           // how many samples the pipeline will use
                mask: !0,                                           // which samples should be active (all in this case)
                alpha_to_coverage_enabled: false,
            },
            multiview: None,                                       // how many array layers the render attachments can have
        });

        // <----- Vertex Buffer ----->
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor
            {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),          // using bytemuck to cast VERTICES as a &[u8]
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // <----- Indices Buffer ----->
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor
            {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(INDICES),          // using bytemuck to cast VERTICES as a &[u8]
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = INDICES.len() as u32;

        // <--------------END-------------->

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
        }
    }

    // Changes the size of the window, through the global state
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    {
        if new_size.width > 0 && new_size.height > 0
        {
            // just change the surface config w/ new dimensions
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // returns a bool to indicate whether an event has been fully processed
    pub fn input(&mut self, event: &WindowEvent) -> bool
    {
        // when cursor moved --->
        // uses cursor position to set self.clear_color
        // this is used in the render pass to describe the window color
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self)
    {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        // get a frame to render to
        let output = self.surface.get_current_texture()?;

        // create TextureView with default settings
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // CommandEncoder to create the actual commands to send to the gpu
        // the encoder builds a command buffer that we can then send to the gpu
        // commands -> command buffer -> gpu
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });


        // render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // where to draw colors to
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,                // what texture to save the colors to (in this case the screen)
                    resolve_target: None,
                    ops: wgpu::Operations {     // tells wgpu what to do with the colors on the screen
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);    // set the pipeline on the render_pass using the one we made in 'new()'
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);        // draw something with num_indices vertices, and 1 instance
        }
    
        // finish the command buffer, and to submit it to the gpu's render queue.
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}
