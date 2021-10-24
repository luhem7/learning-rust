use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};

mod user_interface;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}

unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ]
        }
    }
}



fn main() {
    //main.rs
    // main.rs
    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
    ];

    const INDICES: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];


    struct State {
        surface: wgpu::Surface,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        sc_desc: wgpu::SwapChainDescriptor,
        swap_chain: wgpu::SwapChain,

        clear_color: wgpu::Color,

        render_pipeline: wgpu::RenderPipeline,

        vertex_buffer: wgpu::Buffer,
        num_vertices : u32,

        index_buffer: wgpu::Buffer,
        num_indices : u32,

        size: winit::dpi::PhysicalSize<u32>,

        user_interface : user_interface::UserInterface,
    }

    impl State {
        async fn new(window: &Window) -> Self {
            let size = window.inner_size();

            let surface = wgpu::Surface::create(window);

            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY, // Vulkan + Metal + DX12 + Browser WebGPU
            ).await.unwrap(); // Get used to seeing this
            
            let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            }).await;

            let sc_desc = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            };
            let swap_chain = device.create_swap_chain(&surface, &sc_desc);

            let clear_color = wgpu::Color::BLACK;
    
            let vs_src = include_str!("./shaders/shader_triangle.vert");
            let fs_src = include_str!("./shaders/shader_triangle.frag");

            let vs_spirv = glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
            let fs_spirv = glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();

            let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
            let fs_data = wgpu::read_spirv(fs_spirv).unwrap();

            let vs_module = device.create_shader_module(&vs_data);
            let fs_module = device.create_shader_module(&fs_data);

            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            });
            
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,

                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),

                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),

                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format: sc_desc.format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    },
                ],

                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16, 
                    vertex_buffers: &[
                        Vertex::desc(),
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

            let vertex_buffer = device.create_buffer_with_data(
                bytemuck::cast_slice(VERTICES),
                wgpu::BufferUsage::VERTEX,
            );

            let num_vertices = VERTICES.len() as u32;

            let index_buffer = device.create_buffer_with_data(
                bytemuck::cast_slice(INDICES),
                wgpu::BufferUsage::INDEX,
            );

            let num_indices = INDICES.len() as u32;

            Self {
                surface,
                adapter,
                device,
                queue,
                sc_desc,
                swap_chain,
                clear_color,
                render_pipeline: render_pipeline,
                vertex_buffer : vertex_buffer,
                num_vertices : num_vertices,
                index_buffer : index_buffer,
                num_indices : num_indices,
                size,

                user_interface : user_interface::UserInterface::new(),
            }

        } 

        fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            self.size = new_size;
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }

        fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
            match event {
                WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    self.clear_color = wgpu::Color {
                        r: position.x as f64 / self.size.width as f64,
                        g: position.y as f64 / self.size.height as f64,
                        b: 1.0,
                        a: 1.0,
                    };
                    true
                },
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        } => {
                            let command = self.user_interface.process_key_press(virtual_keycode);
                            match command {
                                user_interface::Command::Continue => true,
                                user_interface::Command::Quit => {
                                    *control_flow = ControlFlow::Exit; 
                                    true
                                }
                                user_interface::Command::NewShape(num_vertices) => {
                                    if num_vertices < 3 {
                                        println!("ERROR! A shape must have greater than 3 vertices");
                                    } else {
                                        println!("Drawing a new shape with {} vertices", num_vertices);

                                    }
                                    true
                                }
                            }
                        }
                        _ => false
                    }
                }
                _ => false,
            }
        }

        fn update(&mut self) {

        }

        fn render(&mut self) {
            let frame = self.swap_chain.get_next_texture()
                .expect("Timeout getting texture");

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: self.clear_color,
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&self.index_buffer, 0, 0);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            drop(render_pass);
            
            self.queue.submit(&[
                encoder.finish()
            ]);
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    use futures::executor::block_on;

    // Since main can't be async, we're going to need to block
    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event, control_flow) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
                
            }
            Event::RedrawRequested(_) => {
                state.update();
                state.render();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
 