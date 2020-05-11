use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};


fn main() {
    struct State {
        surface: wgpu::Surface,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        sc_desc: wgpu::SwapChainDescriptor,
        swap_chain: wgpu::SwapChain,

        clear_color: wgpu::Color,

        render_pipeline: wgpu::RenderPipeline,

        size: winit::dpi::PhysicalSize<u32>,
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

            let vs_src = include_str!("shader.vert");
            let fs_src = include_str!("shader.frag");

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
                    entry_point: "main", // 1.
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
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
                    vertex_buffers: &[],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });


            Self {
                surface,
                adapter,
                device,
                queue,
                sc_desc,
                swap_chain,
                clear_color,
                render_pipeline,
                size,
            }

        }

        fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            self.size = new_size;
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }

        fn input(&mut self, event: &WindowEvent) -> bool {
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
            render_pass.draw(0..3, 0..1);

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
            } if window_id == window.id() => if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CursorMoved {position, ..} => {
                        state.render();
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
 