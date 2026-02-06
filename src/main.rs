use std::time::Instant;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use sysinfo::System;

use agi_core::Core;
use winit::{
    event::{Event, WindowEvent, ElementState, KeyEvent},
    event_loop::{EventLoop},
    window::{Window, WindowBuilder},
    keyboard::{KeyCode, PhysicalKey}
};

use egui_wgpu::Renderer;
use egui_winit::State as EguiState;
use egui::{Color32, RichText, ScrollArea, TextureId, Vec2};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Column {
    pos: [f32; 2],
    state: f32, // 0.0 = inactive, 1.0 = firing
    _padding: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    time: f32,
    resolution_x: f32,
    resolution_y: f32,
    _padding: f32, // Padding to meet 16-byte alignment for uniform buffers
}

enum VisualizationMode {
    BootAnimation,
    EEGPlot,
    MandalaViewer,
}

struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    mode: VisualizationMode,
    // Boot animation specific
    boot_pipeline: wgpu::RenderPipeline,
    boot_bind_group: wgpu::BindGroup,
    column_buffer: wgpu::Buffer,
    // EEG specific
    eeg_pipeline: wgpu::RenderPipeline,
    eeg_bind_group: wgpu::BindGroup,
    eeg_data_buffer: wgpu::Buffer,
    eeg_num_points: u32,
    // Egui Mandala Viewer
    egui_ctx: egui::Context,
    egui_state: EguiState,
    egui_renderer: Renderer,
    mandala_texture: TextureId,
    selected_concept_name: Option<String>,
    // Uniforms
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    start_time: Instant,
    core: Core,
    columns_data: Vec<Column>,
}

impl State {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let boot_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Boot Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let eeg_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("EEG Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("eeg_shader.wgsl").into()),
        });

        let mut core = Core::new();

        // --- Knowledge Ingestion and Indexing ---
        println!("--- Starting knowledge assimilation from files... ---");
        if let Err(e) = core.learn_from_large_file_in_parallel("knowledge.txt", true) {
            eprintln!("Error learning from knowledge.txt: {}", e);
        }
        if let Err(e) = core.learn_from_large_file_in_parallel("identity.txt", true) {
            eprintln!("Error learning from identity.txt: {}", e);
        }
        println!("--- Knowledge assimilation complete. Rebuilding search index... ---");
        core.hippocampus.rebuild_index();
        core.thalamus.rebuild_prototypes();
        println!("--- AGI Core is fully initialized and ready. ---");

        // Dynamically create columns based on the actual number of neurons loaded.
        let num_columns = core.connectome.neurons.len();
        let columns_data: Vec<Column> = (0..num_columns).map(|i| {
            let angle = (i as f32 / num_columns as f32) * 2.0 * std::f32::consts::PI;
            let (sin, cos) = angle.sin_cos();
            Column { 
                pos: [cos * 0.45, sin * 0.45], // Reduced radius to fit the screen
                state: 0.0, // Start inactive
                _padding: 0.0, 
            }
        }).collect();

        let column_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Column Buffer"),
            contents: bytemuck::cast_slice(&columns_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let uniforms = Uniforms {
            time: 0.0,
            resolution_x: size.width as f32,
            resolution_y: size.height as f32,
            _padding: 0.0,
        };
        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let boot_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { // Uniforms
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Column Data
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("boot_bind_group_layout"),
        });

        let eeg_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { // Uniforms
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT, // Now used in fragment for glow
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // EEG Data
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT, // Now used in fragment for glow
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("eeg_bind_group_layout"),
        });

        const EEG_NUM_POINTS: u32 = 1024;
        let eeg_data: Vec<f32> = (0..EEG_NUM_POINTS).map(|i| {
            let angle = (i as f32 / EEG_NUM_POINTS as f32) * std::f32::consts::PI * 8.0;
            angle.sin() * 0.5
        }).collect();

        let eeg_data_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("EEG Data Buffer"),
            contents: bytemuck::cast_slice(&eeg_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let boot_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &boot_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: column_buffer.as_entire_binding(),
                }
            ],
            label: Some("boot_bind_group"),
        });

        let eeg_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &eeg_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: eeg_data_buffer.as_entire_binding(),
                }
            ],
            label: Some("eeg_bind_group"),
        });

        let boot_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Boot Pipeline Layout"),
            bind_group_layouts: &[&boot_bind_group_layout],
            push_constant_ranges: &[],
        });

        let eeg_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("EEG Pipeline Layout"),
            bind_group_layouts: &[&eeg_bind_group_layout],
            push_constant_ranges: &[],
        });

        let boot_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Boot Render Pipeline"),
            layout: Some(&boot_pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &boot_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &boot_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let eeg_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("EEG Render Pipeline"),
            layout: Some(&eeg_pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &eeg_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &eeg_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Enable blending for glow
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Full-screen triangle
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Egui setup
        let egui_ctx = egui::Context::default();
        let egui_state = EguiState::new(egui_ctx.clone(), egui_ctx.viewport_id(), &window, None, None);
        let mut egui_renderer = Renderer::new(&device, config.format, None, 1);

        // Create a placeholder texture for the mandala
        let mandala_wgpu_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Mandala Texture"),
            size: wgpu::Extent3d { width: 512, height: 512, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let mandala_texture_view = mandala_wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mandala_texture = egui_renderer.register_native_texture(&device, &mandala_texture_view, wgpu::FilterMode::Linear);


        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            mode: VisualizationMode::BootAnimation,
            boot_pipeline,
            boot_bind_group,
            column_buffer,
            eeg_pipeline,
            eeg_bind_group,
            eeg_data_buffer,
            eeg_num_points: EEG_NUM_POINTS,
            egui_ctx,
            egui_state,
            egui_renderer,
            mandala_texture,
            selected_concept_name: None,
            uniforms,
            uniform_buffer,
            start_time: Instant::now(),
            core,
            columns_data,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.uniforms.resolution_x = new_size.width as f32;
            self.uniforms.resolution_y = new_size.height as f32;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        let response = self.egui_state.on_window_event(&self.window, event);
        if response.consumed {
            return true;
        }

        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    state: ElementState::Pressed,
                    .. 
                },
                ..
            } => {
                self.mode = match self.mode {
                    VisualizationMode::BootAnimation => VisualizationMode::EEGPlot,
                    VisualizationMode::EEGPlot => VisualizationMode::MandalaViewer,
                    VisualizationMode::MandalaViewer => VisualizationMode::BootAnimation,
                };
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        // This runs every frame, for both modes
        self.core.tick();
        self.uniforms.time = self.start_time.elapsed().as_secs_f32();

        match self.mode {
            VisualizationMode::BootAnimation => {
                // Update columns based on neuron state with a decay effect
                let decay_rate = 0.08; 
                
                // First, apply decay to all columns
                for column in self.columns_data.iter_mut() {
                    column.state = (column.state - decay_rate).max(0.0);
                }

                // Then, set firing neurons' columns to full brightness
                // This now iterates directly over the Vec<Neuron>.
                for neuron in &self.core.connectome.neurons {
                    if neuron.firing {
                        if let Some(column) = self.columns_data.get_mut(neuron.id as usize) {
                            column.state = 1.0; // Set to full brightness
                        }
                    }
                }
                
                // Write the updated column data to the GPU buffer for the shader to use
                self.queue.write_buffer(&self.column_buffer, 0, bytemuck::cast_slice(&self.columns_data));
            }
            VisualizationMode::EEGPlot => {
                // If in EEG mode, update the data buffer to create animation
                let new_eeg_data: Vec<f32> = (0..self.eeg_num_points).map(|i| {
                    let time_offset = self.uniforms.time * 2.0; // Control scroll speed
                    let angle = (i as f32 / 60.0) + time_offset; // Control wave frequency
                    // Combine multiple sine waves for a more organic look
                    let wave1 = angle.sin() * 0.5;
                    let wave2 = (angle * 2.718).sin() * 0.25;
                    let wave3 = (angle * 7.389).sin() * 0.125;
                    (wave1 + wave2 + wave3) / 1.875
                }).collect();
                self.queue.write_buffer(&self.eeg_data_buffer, 0, bytemuck::cast_slice(&new_eeg_data));
            }
        }

        // Update the uniforms buffer, which is used by both shaders
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn update_mandala_texture(&mut self, concept: &Concept) {
        const TEXTURE_SIZE: u32 = 512;
        let trace = &concept.trace;

        let mut image_buffer = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(TEXTURE_SIZE, TEXTURE_SIZE);

        let center_x = TEXTURE_SIZE as f32 / 2.0;
        let center_y = TEXTURE_SIZE as f32 / 2.0;

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;

            let mut intensity = 0.0;
            for (i, &val) in trace.iter().enumerate() {
                let angle = (i as f32 / trace.len() as f32) * 2.0 * std::f32::consts::PI;
                let wave_x = dx * angle.cos() + dy * angle.sin();
                intensity += (wave_x * val * 20.0).cos();
            }

            intensity = (intensity / trace.len() as f32 + 1.0) / 2.0; // Normalize

            let r = (intensity * 255.0) as u8;
            let g = ((1.0 - intensity) * 128.0) as u8;
            let b = ((intensity.sin() * 0.5 + 0.5) * 255.0) as u8;

            *pixel = image::Rgba([r, g, b, 255]);
        }

        let texture_size = wgpu::Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        };

        // Create a new texture and register it with egui
        let new_wgpu_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("New Mandala Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture { 
                texture: &new_wgpu_texture, 
                mip_level: 0, 
                origin: wgpu::Origin3d::ZERO, 
                aspect: wgpu::TextureAspect::All 
            },
            &image_buffer,
            wgpu::ImageDataLayout { 
                offset: 0, 
                bytes_per_row: Some(4 * TEXTURE_SIZE), 
                rows_per_image: Some(TEXTURE_SIZE) 
            },
            texture_size,
        );

        let new_texture_view = new_wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Free the old texture and register the new one
        self.egui_renderer.free_texture(&self.mandala_texture);
        self.mandala_texture = self.egui_renderer.register_native_texture(&self.device, &new_texture_view, wgpu::FilterMode::Linear);
    }


    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        if let VisualizationMode::MandalaViewer = self.mode {
            let raw_input = self.egui_state.take_egui_input(&self.window);
            let full_output = self.egui_ctx.run(raw_input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading(RichText::new("Conceptual Hierarchy Viewer").size(24.0));
                    ui.separator();

                    let concepts = self.core.conceptual_hierarchy.concepts.values().cloned().collect::<Vec<_>>();

                    ui.horizontal(|ui| {
                        // Left panel for concept list
                        ui.vertical(|ui| {
                            ui.set_width(250.0);
                            ui.label(RichText::new("Concepts").strong());
                            ScrollArea::vertical().show(ui, |ui| {
                                for concept in concepts {
                                    if ui.button(concept.name.clone()).clicked() {
                                        self.selected_concept_name = Some(concept.name.clone());
                                        self.update_mandala_texture(&concept);
                                    }
                                }
                            });
                        });

                        // Right panel for mandala visualization
                        ui.separator();
                        ui.vertical(|ui| {
                            if let Some(name) = &self.selected_concept_name {
                                ui.label(RichText::new(format!("Trace: {}", name)).monospace());
                            } else {
                                ui.label("Select a concept to view its trace.");
                            }
                            let available_size = ui.available_size();
                            let image_size = available_size.x.min(available_size.y);
                            ui.image((self.mandala_texture, Vec2::new(image_size, image_size)));
                        });
                    });
                });
            });

            self.egui_state.handle_platform_output(&self.window, full_output.platform_output);
            let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, self.window.scale_factor() as f32);
            
            for (id, image_delta) in &full_output.textures_delta.set {
                self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
            }
            for id in &full_output.textures_delta.free {
                self.egui_renderer.free_texture(id);
            }

            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [self.config.width, self.config.height],
                pixels_per_point: self.window.scale_factor() as f32,
            };

            self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &paint_jobs, &screen_descriptor);

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);

        } else {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })
                ],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            match self.mode {
                VisualizationMode::BootAnimation => {
                    render_pass.set_pipeline(&self.boot_pipeline);
                    render_pass.set_bind_group(0, &self.boot_bind_group, &[]);
                    render_pass.draw(0..3, 0..1); // Draw a full-screen triangle
                }
                VisualizationMode::EEGPlot => {
                    render_pass.set_pipeline(&self.eeg_pipeline);
                    render_pass.set_bind_group(0, &self.eeg_bind_group, &[]);
                    render_pass.draw(0..3, 0..1); // Draw a full-screen triangle
                }
                // This case is unreachable because of the outer if, but required for an exhaustive match
                VisualizationMode::MandalaViewer => {}
            }
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

pub fn main() {
    // Initialize logging
    env_logger::init();

    // Create the event loop and window
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    // Create the state
    let mut state = pollster::block_on(State::new(window.clone()));

    // Start the event loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("Error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // Redraw continuously
                state.window.request_redraw();
            }
            _ => {},
        }
    }).unwrap();
}

    let total_memory_mb = sys.total_memory() / (1024 * 1024);
    let used_memory_mb = sys.used_memory() / (1024 * 1024);

    println!("RAM Usage: {} MB / {} MB", used_memory_mb, total_memory_mb);
    println!("==================================\n");
}
}
