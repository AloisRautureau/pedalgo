//! Used to create a kD representation of a set of constraints to be rendered
use std::mem::size_of_val;
use std::slice::from_raw_parts;

use eframe::glow::HasContext;
use eframe::{egui_glow, glow};
use egui::Vec2;
use crate::Simplex;

pub struct PolyhedronRenderer {
    rendering_program: glow::Program,
    vertex_array: glow::VertexArray,
    buffer: glow::Buffer,

    points: Vec<[f32; 3]>,
    pub view_angle: Vec2,
}

impl PolyhedronRenderer {
    pub fn init(gl: &glow::Context) -> Result<Self, String> {
        let shader_ver = egui_glow::ShaderVersion::get(gl);

        Ok(unsafe {
            let rendering_program = gl.create_program().expect("failed to create program");

            if !shader_ver.is_new_shader_interface() {
                eprintln!("Custom 3D painting hasn't been ported to {shader_ver:?}");
                return Err("aled".to_string());
            }

            let (vertex_shader_src, fragment_shader_src) = (
                r#"
                    uniform vec2 u_view_angle;
                    in vec3 vert;
                    out vec3 out_vert;

                    void main() {
                        gl_Position = vec4(vert, 1.0);
                        gl_Position.x *= cos(u_view_angle.x);
                        //gl_Position.y *= sin(u_view_angle.y);

                        out_vert = vert;
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec3 vert;
                    out vec4 out_color;
                    void main() {
                        out_color = vec4(1.0, 1.0, 0.0, 0.9);
                    }
                "#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_src),
                (glow::FRAGMENT_SHADER, fragment_shader_src),
            ];

            let shaders: Vec<_> = shader_sources
                .into_iter()
                .map(|(shader_type, shader_src)| {
                    let shader = gl
                        .create_shader(shader_type)
                        .expect("failed to create vertex shader");
                    gl.shader_source(
                        shader,
                        &format!("{}\n{}", shader_ver.version_declaration(), shader_src),
                    );
                    gl.compile_shader(shader);
                    gl.attach_shader(rendering_program, shader);
                    shader
                })
                .collect();

            gl.link_program(rendering_program);
            if !gl.get_program_link_status(rendering_program) {
                return Err("failed to link".to_string());
            }

            for shader in shaders {
                gl.detach_shader(rendering_program, shader);
                gl.delete_shader(shader);
            }

            PolyhedronRenderer {
                rendering_program,
                vertex_array: gl.create_vertex_array().expect("failed to create vertex array"),
                buffer: gl.create_buffer().expect("failed to create buffer"),
                points: vec!(),
                view_angle: Vec2::default(),
            }
        })
    }

    pub fn polyhedron_from_constraints(&mut self, simplex: &Simplex) {
        let bfs_points = simplex.bfs_point();
        let mut points = vec!();

        for point in bfs_points {
            let mut td_point = [0.0; 3];
            for (i, v) in point.iter().enumerate() {
                td_point[i] = *v;
            }
            points.push(td_point)
        }

        self.points = points;
        println!("{:?}", self.points)
    }

    pub fn draw(&mut self, gl: &glow::Context, current_point: &[f32; 3]) {
        unsafe {
            // create buffer with polyhedron
            let data = self.points.as_slice();
            let data: &[u8] = from_raw_parts(data.as_ptr().cast(), size_of_val(data));

            self.buffer = gl.create_buffer().expect("could not create buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_size(glow::ARRAY_BUFFER, size_of_val(data) as i32, glow::STATIC_DRAW);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);

            gl.use_program(Some(self.rendering_program));
            gl.uniform_2_f32(
                gl.get_uniform_location(self.rendering_program, "u_view_angle")
                    .as_ref(),
                self.view_angle.x,
                self.view_angle.y,
            );

            gl.bind_vertex_array(Some(self.vertex_array));
            gl.enable_vertex_array_attrib(self.vertex_array, 0);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.vertex_attrib_pointer_f32(0, self.points.len() as i32, glow::FLOAT, false, 0, 0);
            gl.draw_arrays(glow::LINE_LOOP, 0, self.points.len() as i32);
            gl.disable_vertex_attrib_array(0);
        }
    }
}
