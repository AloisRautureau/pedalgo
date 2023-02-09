use crate::constraint::Constraints;
use crate::linear_function::LinearFunction;
use crate::Simplex;
use eframe::Frame;
use egui::{Color32, Context, Style};

#[derive(Default, Debug)]
pub struct SimplexVisualizer {
    function_input: String,
    constraints_input: String,

    simplex: Option<Simplex>,
}

impl eframe::App for SimplexVisualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::Area::new("Linear Program")
            .default_pos(egui::pos2(32f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Linear Program");
                            ui.text_edit_singleline(&mut self.function_input);
                            ui.text_edit_multiline(&mut self.constraints_input);

                            if ui.add(egui::Button::new("RUN")).clicked() {
                                // Parse constraints
                                let mut constraints = Constraints::default();
                                for line in self.constraints_input.lines() {
                                    constraints.add_constraint(
                                        line.parse().expect("invalid constraint input"),
                                    );
                                }

                                // Then create the resulting simplex instance
                                let (command, function) = {
                                    let mut words = self.function_input.split_ascii_whitespace();
                                    let command = words.next();
                                    let function_str =
                                        words.fold(String::new(), |acc, w| acc + w + " ");
                                    (
                                        command,
                                        function_str
                                            .parse::<LinearFunction>()
                                            .unwrap_or(LinearFunction::zero()),
                                    )
                                };

                                self.simplex = match command {
                                    Some("max") => Some(constraints.maximize(&function)),
                                    Some("min") => Some(constraints.minimize(&function)),
                                    _ => None,
                                };
                            }
                        });
                    })
            });

        egui::Area::new("State")
            .default_pos(egui::pos2(512f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.heading("State");
                        ui.vertical(|ui| {
                            if let Some(simplex) = &self.simplex {
                                let current_state = simplex.current_state();
                                ui.colored_label(
                                    Color32::LIGHT_GRAY,
                                    format!("{current_state}"),
                                );
                            } else {
                                ui.colored_label(
                                    Color32::LIGHT_GRAY,
                                    "Press RUN to start the algorithm",
                                );
                            }
                        });
                        if ui.add(egui::Button::new("NEXT STEP")).clicked() {
                            // Parse constraints
                            let mut constraints = Constraints::default();
                            for line in self.constraints_input.lines() {
                                constraints.add_constraint(
                                    line.parse().expect("invalid constraint input"),
                                );
                            }
                    })
            });

        // TODO: Step buttons

        // TODO: Figure display
    }
}
