use crate::constraint::Constraints;
use crate::Simplex;
use eframe::Frame;
use egui::{Color32, Context};

#[derive(Default, Debug)]
pub struct SimplexVisualizer {
    input_text: String,
    constraints: Constraints,
    simplex: Option<Simplex>,
}

impl eframe::App for SimplexVisualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // TODO: Text input panel
        egui::SidePanel::left("linear_program_input").show(ctx, |ui| {
            ui.heading("Linear program:");

            ui.horizontal(|ui| ui.text_edit_multiline(&mut self.input_text))
        });

        // TODO: State display
        egui::SidePanel::right("state_display").show(ctx, |ui| {
            ui.heading("State:");

            ui.horizontal(|ui| {
                if let Some(simplex) = &self.simplex {
                    let current_state = simplex.current_state();
                    ui.colored_label(
                        Color32::RED,
                        format!("max {}", current_state.linear_function),
                    );
                    for constraint in current_state.constraints.iter() {
                        ui.label(constraint.to_string());
                    }
                } else {
                    ui.colored_label(Color32::LIGHT_GRAY, "Press RUN to start the algorithm");
                }
            })
        });

        // TODO: Step buttons

        // TODO: Figure display
    }
}
