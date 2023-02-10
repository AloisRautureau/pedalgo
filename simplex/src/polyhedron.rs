//! Used to create a kD representation of a set of constraints to be rendered
use crate::constraint::Constraints;
use eframe::glow::HasContext;
use eframe::{egui_glow, glow};
use egui::Vec2;

pub struct PolyhedronRenderer {
    rendering_program: glow::Program,
    vertex_array: glow::VertexArray,
    pub view_angle: Vec2,
}

impl PolyhedronRenderer {
    pub fn init(gl: &glow::Context) -> Result<Self, ()> {
        let shader_ver = egui_glow::ShaderVersion::get(gl);

        Ok(unsafe {
            let rendering_program = gl.create_program().expect("failed to create program");

            if !shader_ver.is_new_shader_interface() {
                eprintln!("Custom 3D painting hasn't been proted to {shader_ver:?}");
                return Err(());
            }

            let (vertex_shader_src, fragment_shader_src) = (
                r#"
                    const vec3 verts[3] = vec3[3](
                        vec3(0.0, 1.0, 0.0),
                        vec3(-1.0, -1.0, 0.0),
                        vec3(1.0, -1.0, 0.0)
                    );
                    uniform vec2 u_view_angle;
                    void main() {
                        gl_Position = vec4(verts[gl_VertexID], 1.0);
                        gl_Position.x *= cos(u_view_angle.x);
                        gl_Position.y *= sin(u_view_angle.y);
                    }
                "#,
                r#"
                    precision mediump float;
                    out vec4 out_color;
                    void main() {
                        out_color = vec4(0.27, 0.52, 0.53, 0.9);
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
                return Err(());
            }

            for shader in shaders {
                gl.detach_shader(rendering_program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("cannot create vertex array");

            PolyhedronRenderer {
                rendering_program,
                vertex_array,
                view_angle: Vec2::default(),
            }
        })
    }

    pub fn polyhedron_from_constraints(&mut self, _constraints: &Constraints) {
        todo!()
    }

    pub fn draw(&self, gl: &glow::Context, _current_point: ()) {
        unsafe {
            gl.use_program(Some(self.rendering_program));
            gl.uniform_2_f32(
                gl.get_uniform_location(self.rendering_program, "u_view_angle")
                    .as_ref(),
                self.view_angle.x,
                self.view_angle.y,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::LINE_LOOP, 0, 3)
        }
    }
}
