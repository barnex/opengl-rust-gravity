extern crate gl_img;
extern crate gl_safe;
extern crate gl_win;
extern crate image;
extern crate rand;
extern crate structopt;

use gl::*;
use gl_obj::*;
use gl_safe::*;
use gl_win::*;
use glutin::event::ElementState;
use glutin::event::MouseButton;
use glutin::event::MouseScrollDelta;
use glutin::event::VirtualKeyCode;
use rand::prelude::*;
use std::cell::Cell;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time;
use structopt::StructOpt;

/// OpenGL water simulation.
#[derive(StructOpt)]
struct Args {
	/// Image width (pixels).
	#[structopt(short, long, default_value = "1024")]
	width: u32,

	/// Image height, (pixels).
	#[structopt(short, long, default_value = "1024")]
	height: u32,

	/// Verlet integration time step.
	#[structopt(long, default_value = "0.001")]
	dt: f32,

	/// Time steps per rendered frame.
	#[structopt(short, long, default_value = "10")]
	steps_per_frame: u32,

	/// Render scaling
	#[structopt(long, default_value = "200.0")]
	scale: f32,
}

fn main() {
	let args = Args::from_args();

	// window
	let size = uvec2(args.width, args.height);
	let (w, h) = (size.0, size.1);
	let (win, ev) = init_gl_window(w, h, "gravity");

	let s = State::new(&args);

	// continuously pump redraws
	let proxy = ev.create_proxy();
	std::thread::spawn(move || loop {
		proxy.send_event(()).expect("send event");
		std::thread::sleep(time::Duration::from_millis(6));
	});

	// infinite event loop
	run_event_loop(ev, win, s);
}

struct State {
	scale: f32,
	p_render: Program,
	p_density: Program,
	p_decay: Program,
	pos: Texture,
	density: Texture,
	vao: VertexArray,
	time_steps_per_draw: u32,
	mouse_down: bool,
	frames: Cell<i32>,
}

impl State {
	fn new(args: &Args) -> Self {
		let (pos, vel) = Self::initial_particles(&args);
		let size = uvec2(args.width, args.height);
		let p_render = Program::new(&[
			//
			Shader::new_vert(include_str!("texture.vert")),
			Shader::new_frag(include_str!("draw.frag")),
		]);

		Self {
			scale: args.scale,
			p_decay: Self::compute_prog(include_str!("decay.glsl")),
			p_density: Self::compute_prog(include_str!("density.glsl")),
			p_render,
			pos: Self::vec_to_tex(size, &pos),
			density: Texture::new2d(RGBA8UI, size).filter_nearest(),
			vao: Self::vao(p_render),
			time_steps_per_draw: args.steps_per_frame,
			frames: Cell::new(0),
			mouse_down: false,
		}
	}

	fn vec_to_tex(size: uvec2, data: &[vec2]) -> Texture {
		Texture::new2d(RG32F, size).sub_image2d(0, 0, 0, size.0, size.1, RG, FLOAT, data)
	}

	fn initial_particles(args: &Args) -> (Vec<vec2>, Vec<vec2>) {
		let (w, h) = (args.width, args.height);

		let mut pos = Vec::<vec2>::with_capacity((w * h) as usize);
		let mut vel = Vec::<vec2>::with_capacity((w * h) as usize);

		let mut rng = rand::thread_rng();
		let mut urand = move || rng.gen::<f32>();
		let mut irand = move || rng.gen::<f32>() - 0.5;

		for _y in 0..h {
			for _x in 0..w {
				let th = 0.5 * PI * irand();
				let r = 0.5 * urand() + 1.3;
				let x = r * th.cos();
				let y = r * th.sin();
				pos.push(vec2(x, y));
				vel.push(vec2(y / r + 0.01 * irand(), -x / r - 0.01 * irand())); // TODO
			}
		}
		(pos, vel)
	}

	fn steps(&mut self, n: u32) {
		// TODO
		self.update_density();
	}

	fn update_density(&self) {
		self.density.bind_image_unit(0, READ_WRITE);
		self.exec(self.p_decay);

		self.p_density.set1f("scale", self.scale);
		self.pos.bind_image_unit(0, READ_WRITE); // TODO
		self.density.bind_image_unit(1, READ_WRITE);
		self.exec(self.p_density);
	}

	fn draw(&self, _w: &Window) {
		glClearColor(0.5, 0.5, 0.5, 1.0);
		glClear(gl::COLOR_BUFFER_BIT);

		self.p_render.use_program();
		self.vao.bind();
		self.density.bind_texture_unit(3);

		glDrawArrays(gl::TRIANGLE_STRIP, 0, 4);
	}

	fn exec(&self, p: Program) {
		let xy = self.pos.size();
		p.compute_and_sync(uvec3(xy.0, xy.1, 1))
	}

	fn on_cursor_moved(&self, position: (f64, f64)) {
		if self.mouse_down {
			let (w, h) = (self.pos.size().0 as i32, self.pos.size().1 as i32);
			let (x, y) = ((position.0) as i32, (position.1) as i32);
			if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
				let x = (x - w / 2) as f32;
				let y = (y - h / 2) as f32;
				let x = x / self.scale;
				let y = y / self.scale;
				//self.p_accel.set2f("sun_pos", x, y);
			}
		}
	}

	fn on_mouse_input(&mut self, _button: MouseButton, state: ElementState) {
		self.mouse_down = match state {
			ElementState::Pressed => true,
			ElementState::Released => false,
		};
	}

	fn on_redraw_requested(&mut self, win: &Window) {
		self.draw(&win);
		win.swap_buffers().unwrap();
		self.steps(self.time_steps_per_draw);
		self.frames.set(self.frames.get() + 1);
		//let secs = self.start.elapsed().as_secs_f32();
		//let fps = self.frames.get() as f32 / secs;
		//dbg!(fps);
	}

	fn zoom(&mut self, scale: f32) {
		self.scale = self.scale * scale;
		self.density.bind_image_unit(0, READ_WRITE);
	}

	fn on_user_event(&self, win: &Window) {
		win.window().request_redraw()
	}

	fn on_cursor_entered(&self) {}

	fn on_cursor_left(&self) {}

	fn on_mouse_wheel(&mut self, delta: MouseScrollDelta) {
		let mut zoom = |dy| {
			if dy > 0.0 {
				self.zoom(1.05)
			}
			if dy < 0.0 {
				self.zoom(0.95)
			}
		};
		match delta {
			MouseScrollDelta::LineDelta(_x, y) => zoom(y),
			MouseScrollDelta::PixelDelta(phys) => zoom(phys.y as f32),
		}
	}

	fn on_key(&mut self, k: VirtualKeyCode) {
		println!("key {:?}", k);
		//use VirtualKeyCode::*;
		match k {
			VirtualKeyCode::Equals | VirtualKeyCode::Plus => self.zoom(2.0),
			VirtualKeyCode::Minus | VirtualKeyCode::Underline => self.zoom(0.5),
			_ => (),
		}
	}

	fn compute_prog(src: &str) -> Program {
		Program::new(&[Shader::new_comp(src)])
	}

	fn vao(prog: Program) -> VertexArray {
		let v_pos = [
			//
			vec2(-1.0, 1.0),
			vec2(-1.0, -1.0),
			vec2(1.0, 1.0),
			vec2(1.0, -1.0),
		];
		let v_pos_buf = Buffer::new(&v_pos, 0);

		let v_texc = [
			//
			vec2(0.0, 0.0),
			vec2(0.0, 1.0),
			vec2(1.0, 0.0),
			vec2(1.0, 1.0),
		];
		let v_texc_buf = Buffer::new(&v_texc, 0);

		let v_pos_attr = prog.attrib_location("vertex_pos").unwrap();
		let v_texc_attr = prog.attrib_location("vertex_tex_coord").unwrap();
		VertexArray::create()
			.enable_attrib(v_pos_attr)
			.attrib_format(v_pos_attr, 2, gl::FLOAT, false, 0)
			.vertex_buffer(v_pos_attr, v_pos_buf, 0, sizeof(v_pos[0]))
			.enable_attrib(v_texc_attr)
			.attrib_format(v_texc_attr, 2, gl::FLOAT, false, 0)
			.vertex_buffer(v_texc_attr, v_texc_buf, 0, sizeof(v_texc[0]))
	}
}

fn run_event_loop(ev: EventLoop, win: Arc<Window>, mut s: State) {
	ev.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::RedrawRequested(_) => s.on_redraw_requested(&win),
			Event::UserEvent(_) => s.on_user_event(&win),
			Event::LoopDestroyed => return,
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CursorMoved { position, .. } => s.on_cursor_moved((position.x, position.y)),
				WindowEvent::MouseInput { state, button, .. } => s.on_mouse_input(button, state),
				WindowEvent::MouseWheel { delta, .. } => s.on_mouse_wheel(delta),
				WindowEvent::CursorEntered { .. } => s.on_cursor_entered(),
				WindowEvent::CursorLeft { .. } => s.on_cursor_left(),
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(k) = input.virtual_keycode {
						s.on_key(k)
					}
				}
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_ => (),
			},
			_ => (),
		}
	});
}
