use crate::{State, VisualizationMode};

use egui::{ScrollArea, Vec2};

pub fn draw_ui(ctx: &egui::Context, state: &mut State) {
    // --- Left Panel (Controls) ---
    egui::SidePanel::left("control_panel")
        .resizable(true)
        .default_width(250.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("NeuroVisualizer");
                ui.separator();

            // Performance metrics are fetched in the main update loop now
            // to avoid locking the core multiple times.
            let (tps, power) = {
                let core_lock = state.core.lock().unwrap();
                (
                    core_lock.processing_speed.load(std::sync::atomic::Ordering::Relaxed),
                    core_lock.power_draw.load(std::sync::atomic::Ordering::Relaxed)
                )
            };

            ui.label(format!("AGI Core TPS: {:.2}", tps));
            ui.label(format!("Power Draw: {:.2} W", power));
            ui.separator();

            ui.label("Visualization Mode:");
            ui.radio_value(&mut state.mode, VisualizationMode::BootAnimation, "Boot Animation (B)");
            ui.radio_value(&mut state.mode, VisualizationMode::EEGPlot, "EEG Plot (E)");
            ui.radio_value(&mut state.mode, VisualizationMode::MandalaViewer, "Mandala Viewer (M)");
            ui.separator();

            // --- Deep Thinker UI (Disabled) ---

            // --- Mandala-specific controls ---
            if state.mode == VisualizationMode::MandalaViewer {
                ui.heading("Conceptual Hierarchy");
                let concepts = state.core.lock().unwrap().conceptual_hierarchy.get_all_concept_names();
                ScrollArea::vertical().show(ui, |ui| {
                    let mut new_selection_name: Option<String> = None;
                    for name_str in &concepts {
                        let is_selected = state.selected_concept_name.as_deref() == Some(name_str.as_str());
                        if ui.selectable_label(is_selected, name_str).clicked() {
                            state.selected_concept_name = Some(name_str.to_string());
                            new_selection_name = Some(name_str.to_string());
                        }
                    }
                    if let Some(name) = new_selection_name {
                        let concept_to_update = {
                            state.core.lock().unwrap().conceptual_hierarchy.find_concept_by_name(&name).cloned()
                        };
                        if let Some(concept) = concept_to_update {
                            state.update_mandala_texture(&concept);
                        }
                    }
                });
            }
        }); // Closes ScrollArea
    });

    // --- Right Panel (Chat) ---
    egui::SidePanel::right("chat_panel")
        .resizable(true)
        .default_width(350.0)
        .show(ctx, |ui| {
            ui.heading("Conversation");
            ui.separator();
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.vertical(|ui| {
                    for line in &state.chat_history {
                        ui.label(line);
                    }
                });
            });
        });



    // --- Central Panel (Mandala View or other main content) ---
    // Make the central panel transparent so the background wgpu rendering is visible
    let frame = egui::Frame::none();
    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        if state.mode == VisualizationMode::MandalaViewer {
            if let Some(name) = &state.selected_concept_name {
                ui.heading(format!("Holographic Trace: {}", name));
                let available_size = ui.available_size();
                let image_size = available_size.x.min(available_size.y);
                ui.image((state.mandala_texture, Vec2::new(image_size, image_size)));
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a concept to view its trace.");
                });
            }
        }
    });

    // --- Bottom Panel (AGI Interaction) ---
    egui::TopBottomPanel::bottom("prompt_panel")
        .resizable(false)
        .min_height(40.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let prompt_input = ui.add(egui::TextEdit::singleline(&mut state.prompt_buffer).hint_text("Enter your prompt...").id(egui::Id::new("prompt_input")).desired_width(f32::INFINITY));
                let enter_pressed = prompt_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Send").clicked() || enter_pressed {
                    if !state.prompt_buffer.is_empty() {
                        let prompt = state.prompt_buffer.trim().to_string();
                        state.chat_history.push(format!("You: {}", prompt));
                                                                        if let Some((response, _)) = state.core.lock().unwrap().get_response_for_prompt(&prompt) {
                            state.chat_history.push(format!("AGI: {}", response));
                        }
                        state.prompt_buffer.clear();
                        prompt_input.request_focus();
                    }
                }
            });
        });
}
