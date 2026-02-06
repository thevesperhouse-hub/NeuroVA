use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;
use wgpu::util::DeviceExt;
use sysinfo::System;

use agi_core::{Core, conceptual_hierarchy::ConceptNode};


use winit::{
    event::{Event, WindowEvent, ElementState, KeyEvent},
    event_loop::{EventLoop},
    window::{Window, WindowBuilder},
    keyboard::{KeyCode, PhysicalKey}
};

use egui_wgpu::Renderer;
use egui_winit::State as EguiState;
use egui::{TextureId};

mod gui;

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
    awareness_level: f32, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisualizationMode {
    BootAnimation,
    EEGPlot,
    MandalaViewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    WakingUp,
    Running,
}

struct State {
    // deep_thought_query: String,
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    app_state: AppState,
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
    last_wakeup_time: Instant,
    // AGI Core and UI State
    core: Arc<Mutex<Core>>,
    columns_data: Vec<Column>,
    prompt_buffer: String,
    agi_response: String, // Still used for the last raw response
    chat_history: Vec<String>,
}

impl State {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

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
            },
            None,
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
            present_mode: wgpu::PresentMode::Fifo,
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

        let core = { 
            let mut core = Core::new(None);
            core.set_wakeup_stages(5); // Start the wakeup sequence
            // --- AGI Consciousness Seeding ---
            // Load foundational knowledge from external files.
            println!("--- Seeding AGI consciousness... ---");

            // Use CARGO_MANIFEST_DIR to create robust paths to the knowledge files.
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let base_path = std::path::Path::new(manifest_dir).parent().unwrap();

            // 1. Load identity
            let identity_path = base_path.join("identity.txt");
            if let Err(e) = core.learn_from_file(identity_path.to_str().unwrap()) {
                eprintln!("FATAL: Could not load identity.txt from {:?}: {}", identity_path, e);
            } else {
                println!("[OK] Identity loaded.");
            }

            // 2. Load knowledge base
            let knowledge_path = base_path.join("knowledge.txt");
            if let Err(e) = core.learn_from_file(knowledge_path.to_str().unwrap()) {
                eprintln!("FATAL: Could not load knowledge.txt from {:?}: {}", knowledge_path, e);
            } else {
                println!("[OK] Knowledge base loaded.");
            }
            
            // 3. Assimilate all loaded knowledge into the holographic memory
            println!("--- Assimilating knowledge... ---");
            core.assimilate_knowledge();
            println!("[OK] Knowledge assimilated into holographic memory.");
            Arc::new(Mutex::new(core))
        };

        // Spawn AGI thread
        let agi_core_clone = Arc::clone(&core);
        thread::spawn(move || {
            loop {
                let mut agi_core = agi_core_clone.lock().unwrap();
                agi_core.tick();
                // We can drop the lock here to allow the main thread to access the core
                drop(agi_core);
                thread::sleep(Duration::from_millis(10)); // Tick rate
            }
        });

        // Dynamically create columns based on the actual number of neurons loaded.
        let num_columns = core.lock().unwrap().connectome.neurons.len();
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
            awareness_level: 0.0,
        };
        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // Define all bind group layouts first
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
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // EEG Data
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
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

        // Define all pipeline layouts
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

        // Create buffers
        const EEG_NUM_POINTS: u32 = 1024;
        let eeg_data: Vec<f32> = vec![0.0; EEG_NUM_POINTS as usize];
        let eeg_data_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("EEG Data Buffer"),
            contents: bytemuck::cast_slice(&eeg_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind groups
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

        // Create pipelines
        let boot_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Boot Render Pipeline"),
            layout: Some(&boot_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &boot_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &boot_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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
            vertex: wgpu::VertexState {
                module: &eeg_shader,
                entry_point: "vs_main",
                buffers: &[], // No vertex buffers needed, vertices are generated in the shader
            },
            fragment: Some(wgpu::FragmentState {
                module: &eeg_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Enable blending for glow
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
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
            // deep_thought_query: String::new(),
            window,
            surface,
            device,
            queue,
            config,
            size,
            app_state: AppState::WakingUp,
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
            last_wakeup_time: Instant::now(),
            core,
            columns_data,
            prompt_buffer: String::new(),
            chat_history: Vec::new(),
            agi_response: "AGI is initializing...".to_string(),
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
                    physical_key: PhysicalKey::Code(key_code), 
                    state: ElementState::Pressed, 
                    .. 
                }, 
                ..
            } => {
                // If egui wants keyboard input, don't process our own shortcuts
                if self.egui_ctx.wants_keyboard_input() {
                    return false;
                }

                match key_code {
                    KeyCode::KeyB => self.mode = VisualizationMode::BootAnimation,
                    KeyCode::KeyE => self.mode = VisualizationMode::EEGPlot,
                    KeyCode::KeyM => self.mode = VisualizationMode::MandalaViewer,
                    _ => return false, // Return false for unhandled keys
                }
                true // Return true because we handled the input
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        // Check for new AGI response and clear it from the core to prevent spam
        // Lock the core once and perform all necessary updates
        // --- High-Priority: AGI Response Handling ---
        // We use a blocking lock here to ensure we *always* process a response if one is ready.
        // This prevents race conditions where the UI thread misses a response because the AGI thread has the lock.
        {
            let mut core = self.core.lock().unwrap();
            if let Some(response) = core.get_response() {
                if !response.is_empty() {
                    let formatted_response = format!("AGI: {}", response);
                    if self.chat_history.last().map_or(true, |last| last != &formatted_response) {
                        self.chat_history.push(formatted_response);
                        self.agi_response = response;
                    }
                    // This is now redundant as get_response consumes the result, but kept for clarity.
                    core.clear_response(); 
                }
            }
        }

        // --- Lower-Priority: Visualization Updates ---
        // We use a non-blocking `try_lock` here. If the core is busy, we'll just skip
        // updating the visuals for one frame. This keeps the UI responsive.
        self.uniforms.time = self.start_time.elapsed().as_secs_f32();

        match self.app_state {
            AppState::WakingUp => {
                if self.last_wakeup_time.elapsed() >= std::time::Duration::from_millis(200) {
                    let mut core = self.core.lock().unwrap();
                    if !core.advance_wakeup_stage() {
                        self.app_state = AppState::Running;
                        self.mode = VisualizationMode::MandalaViewer; // Switch to Mandala view after wakeup
                        println!("Wakeup sequence complete. Switching to Mandala viewer.");
                    }
                    self.last_wakeup_time = Instant::now();
                }
                // Update awareness level for the shader
                self.uniforms.awareness_level = self.core.lock().unwrap().get_awakening_level();
            }
            AppState::Running => {
                if let Ok(core) = self.core.try_lock() {
                    match self.mode {
                        VisualizationMode::BootAnimation => {
                            // This mode should not be active in Running state, but as a fallback:
                            for (i, neuron) in core.connectome.neurons.iter().enumerate() {
                                if let Some(column) = self.columns_data.get_mut(i) {
                                    column.state = if neuron.firing {
                                        1.0
                                    } else {
                                        (column.state * 0.95).max(neuron.potential / 1.0)
                                    };
                                }
                            }
                            self.queue.write_buffer(&self.column_buffer, 0, bytemuck::cast_slice(&self.columns_data));
                        }
                        VisualizationMode::EEGPlot => {
                            let eeg_data = core.get_eeg_potentials(self.eeg_num_points as usize);
                            self.queue.write_buffer(&self.eeg_data_buffer, 0, bytemuck::cast_slice(&eeg_data));
                        }
                        VisualizationMode::MandalaViewer => {}
                    }
                }
            }
        }

        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn update_mandala_texture(&mut self, concept: &ConceptNode) {
        const TEXTURE_SIZE: u32 = 512;
        let trace = &concept.trace;

        let mut image_buffer = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(TEXTURE_SIZE, TEXTURE_SIZE);

        let center_x = TEXTURE_SIZE as f32 / 2.0;
        let center_y = TEXTURE_SIZE as f32 / 2.0;

        if !trace.weighted_concepts.is_empty() {
            let total_points: usize = trace.weighted_concepts.values().map(|c| c.interference_pattern.len()).sum();
            for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                let mut intensity = 0.0;
                for weighted_concept in trace.weighted_concepts.values() {
                    for (i, point) in weighted_concept.interference_pattern.iter().enumerate() {
                        let dx = x as f32 - center_x;
                        let dy = y as f32 - center_y;
                        let radius = (dx * dx + dy * dy).sqrt();

                        // Simple interference calculation, modulated by concept relevance.
                        let wave = (radius * 0.1 + point.re * 10.0 + point.im * 5.0 + (i as f32) * 0.05).sin() * weighted_concept.relevance;
                        intensity += wave;
                    }
                }

                // Normalize intensity based on the total number of points across all concepts.
                let normalized_intensity = if total_points > 0 {
                    (intensity / (total_points as f32 * 0.5) + 1.0) / 2.0
                } else {
                    0.0
                };
                let color_val = (normalized_intensity.clamp(0.0, 1.0) * 255.0) as u8;
                *pixel = image::Rgba([color_val, (color_val as f32 * 0.7) as u8, (color_val as f32 * 0.5) as u8, 255]);
            }
        } else {
            // If trace is empty, create a blank texture.
            for pixel in image_buffer.pixels_mut() {
                *pixel = image::Rgba([0, 0, 0, 255]);
            }
        }

        let texture_size = wgpu::Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        };

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
                rows_per_image: Some(TEXTURE_SIZE), 
            },
            texture_size,
        );

        let new_texture_view = new_wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Unregister the old texture before registering the new one
        self.egui_renderer.free_texture(&self.mandala_texture);
        self.mandala_texture = self.egui_renderer.register_native_texture(&self.device, &new_texture_view, wgpu::FilterMode::Linear);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Egui: Get UI definition
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let egui_ctx = self.egui_ctx.clone();
        let full_output = egui_ctx.run(raw_input, |ctx| {
            gui::draw_ui(ctx, self);
        });

        // Egui: Handle platform output and tessellate shapes
        self.egui_state.handle_platform_output(&self.window, full_output.platform_output);
        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, self.window.scale_factor() as f32);

        // Egui: Update textures
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
        }
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        // Create screen descriptor for egui
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        // Egui: Update buffers
        self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &paint_jobs, &screen_descriptor);

        {
            // Main render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
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

            // Draw the background visualization first
            match self.mode {
                VisualizationMode::BootAnimation => {
                    render_pass.set_pipeline(&self.boot_pipeline);
                    render_pass.set_bind_group(0, &self.boot_bind_group, &[]);
                    render_pass.draw(0..6, 0..1); // Draw a quad
                }
                VisualizationMode::EEGPlot => {
                    render_pass.set_pipeline(&self.eeg_pipeline);
                    render_pass.set_bind_group(0, &self.eeg_bind_group, &[]);
                    render_pass.draw(0..self.eeg_num_points, 0..1);
                }
                // For MandalaViewer, the background is clear, and the mandala is in the UI
                VisualizationMode::MandalaViewer => {}
            }

            // Draw Egui on top
            self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}



pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let mut state = State::new(window.clone()).await;

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
                        WindowEvent::ScaleFactorChanged { .. } => {
                            //TODO
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => (),
        }
    }).unwrap();
}

fn main() {
    env_logger::init();
    println!("Starting NeuroVA...");

    // System info
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory_mb = sys.total_memory() / (1024 * 1024);
    let used_memory_mb = sys.used_memory() / (1024 * 1024);

    println!("RAM Usage: {} MB / {} MB", used_memory_mb, total_memory_mb);

    pollster::block_on(run());
}

