use crate::constraint::Constraints;
use crate::linear_function::LinearFunction;
use crate::Simplex;
use eframe::Frame;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::Body;
use egui::TextStyle::Button;
use egui::TextStyle::Heading;
use egui::TextStyle::Monospace;
use egui::TextStyle::Small;
use egui::{Color32, Context, Style};

#[derive(Debug)]
pub struct SimplexVisualizer {
    maximize: bool,
    function_input: String,
    constraints_input: String,

    simplex: Option<Simplex>,
}
impl Default for SimplexVisualizer {
    fn default() -> Self {
        SimplexVisualizer {
            maximize: true,
            function_input: String::from("x + 6y + 13z"),
            constraints_input: String::from(
                "\
x <= 200\n\
y <= 300\n\
x + y + z <= 400\n\
y + 3z <= 600\n
            ",
            ),
            simplex: None,
        }
    }
}
impl eframe::App for SimplexVisualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Change font sizes
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(24.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        egui::Area::new("Linear Program")
            .default_pos(egui::pos2(32f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Linear Program");
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_label("")
                                    .selected_text(format!(
                                        "{}",
                                        if self.maximize { "MAX" } else { "MIN" }
                                    ))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.maximize, true, "MAX");
                                        ui.selectable_value(&mut self.maximize, false, "MIN");
                                    });
                                ui.text_edit_singleline(&mut self.function_input);
                            });
                            ui.text_edit_multiline(&mut self.constraints_input);

                            if ui.add(egui::Button::new("RUN")).clicked() {
                                // Parse constraints
                                let constraints = Constraints::compile(&self.constraints_input).unwrap();
                                // Parse linear function
                                let function = self
                                .function_input
                                .parse()
                                .unwrap_or(LinearFunction::zero());

                                // Run simplex
                                self.simplex = Some(constraints.maximize(&if self.maximize {
                                    function
                                } else {
                                    -function
                                }));
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
                                ui.colored_label(Color32::LIGHT_GRAY, format!("{current_state}"));
                            } else {
                                ui.colored_label(
                                    Color32::LIGHT_GRAY,
                                    "Press RUN to start the algorithm",
                                );
                            }
                        });
                        ui.horizontal_centered(|ui| {
                            // Previous button
                            if ui.add(egui::Button::new("PREVIOUS")).clicked() {
                                if let Some(simplex) = &mut self.simplex {
                                    simplex.previous_step();
                                }
                            }
                            // Next button
                            if ui.add(egui::Button::new("NEXT")).clicked() {
                                if let Some(simplex) = &mut self.simplex {
                                    simplex.next_step(true);
                                }
                            }
                        })
                    })
            });

        // TODO: Step buttons

        // TODO: Figure display
    }
}
