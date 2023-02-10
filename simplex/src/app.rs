use crate::constraint::Constraints;
use crate::linear_function::LinearFunction;
use crate::polyhedron::PolyhedronRenderer;
use crate::Simplex;
use eframe::{egui_glow, Frame};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use egui::{Color32, Context, Style};
use std::sync::{Arc, Mutex};

const _BLUE: Color32 = Color32::from_rgb(69, 133, 136);
const _RED: Color32 = Color32::from_rgb(204, 36, 29);
const _BG: Color32 = Color32::from_rgb(40, 40, 40);
const _FG: Color32 = Color32::from_rgb(235, 219, 178);

pub struct SimplexVisualizer {
    maximize: bool,
    function_input: String,
    constraints_input: String,

    simplex: Option<Simplex>,
    polyhedron_renderer: Arc<Mutex<PolyhedronRenderer>>,
}

impl SimplexVisualizer {
    pub fn init(cc: &eframe::CreationContext) -> SimplexVisualizer {
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
            polyhedron_renderer: Arc::new(Mutex::new(
                PolyhedronRenderer::init(cc.gl.as_ref().unwrap()).unwrap(),
            )),
        }
    }

    fn draw_polyhedron(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_at_least(egui::Vec2::splat(300.0), egui::Sense::drag());

        // Check angle
        self.polyhedron_renderer.lock().unwrap().view_angle += response.drag_delta() * 0.01;

        let polyhedron_renderer = self.polyhedron_renderer.clone();
        let callback = Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
            polyhedron_renderer.lock().unwrap().draw(painter.gl(), ())
        }));
        let callback = egui::PaintCallback { rect, callback };
        ui.painter().add(callback);
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
                                    .selected_text((if self.maximize { "MAX" } else { "MIN" }).to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.maximize, true, "MAX");
                                        ui.selectable_value(&mut self.maximize, false, "MIN");
                                    });
                                ui.text_edit_singleline(&mut self.function_input);
                            });
                            ui.text_edit_multiline(&mut self.constraints_input);

                            if ui.add(egui::Button::new("COMPILE")).clicked() {
                                // Parse constraints
                                let constraints =
                                    Constraints::compile(&self.constraints_input).unwrap();
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

                        ui.horizontal(|ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| self.draw_polyhedron(ui));
    }
}
