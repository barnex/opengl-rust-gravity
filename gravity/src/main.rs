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
use rand::prelude::*;
use std::cell::Cell;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time;
use structopt::StructOpt;

const MIN_POW: f32 = 0.05;
const MAX_POW: f32 = 0.2;

/// OpenGL water simulation.
#[derive(StructOpt)]
struct Args {
	/// Image width (pixels).
	#[structopt(short, long, default_value = "1024")]
	width: u32,

	/// Image height, (pixels).
	#[structopt(short, long, default_value = "512")]
	height: u32,

	/// Verlet integration time step.
	#[structopt(long, default_value = "0.001")]
	dt: f32,

	/// Radius of mouse disturbance.
	#[structopt(long, default_value = "50")]
	mouse_radius: f32,
}

fn main() {
	let args = Args::from_args();

	// window
	let size = uvec2(args.width, args.height);
	let (w, h) = (size.0, size.1);
	let (win, ev) = init_gl_window(w, h, "waves");

	// water state
	let s = State::new(&args);

	//s.p_accel //
	//.set1f("damping", args.damping);
	//s.p_verlet //
	//.set1f("dt", args.dt);

	//s.p_mouse //
	//.set1f("mouse_rad", args.mouse_radius);

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
	p_accel: Program,
	p_verlet: Program,
	p_mouse: Program,
	p_render: Program,
	p_photon: Program,
	p_decay: Program,
	pos: Texture,
	vel: Texture,
	acc: Texture,
	photon: Texture,
	vao: VertexArray,
	time_steps_per_draw: u32,
	start: time::Instant,
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
			p_accel: Self::compute_prog(include_str!("accel.glsl")),
			p_verlet: Self::compute_prog(include_str!("verlet.glsl")),
			p_mouse: Self::compute_prog(include_str!("apply_mouse.glsl")),
			p_decay: Self::compute_prog(include_str!("udecay.glsl")),
			p_photon: Self::compute_prog(include_str!("render.glsl")),
			p_render,
			pos: Self::vec_to_tex(size, &pos),
			vel: Self::vec_to_tex(size, &vel),
			acc: Texture::new2d(RG32F, size),
			photon: Texture::new2d(RGBA8UI, size).filter_nearest(),
			vao: Self::vao(p_render),
			time_steps_per_draw: 6,
			start: time::Instant::now(),
			frames: Cell::new(0),
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
		let mut rand = move || rng.gen::<f32>();

		for y in 0..h {
			for x in 0..w {
				let th = 2.0 * PI * rand();
				let r = rand() + 0.2;
				let x = r * th.cos();
				let y = r * th.sin();
				pos.push(vec2(x, y));
				vel.push(vec2(y, -x)); // TODO
			}
		}
		(pos, vel)
	}

	fn steps(&mut self, n: u32) {
		for _ in 0..n {
			self.update_acc();
			self.update_pos_vel();
			//	self.apply_mouse();
		}
		self.update_photon();
	}

	fn update_acc(&self) {
		self.pos.bind_image_unit(0, READ_ONLY);
		self.acc.bind_image_unit(1, WRITE_ONLY);
		//self.acc.bind_image_unit(2, WRITE_ONLY);
		self.exec(self.p_accel)
	}

	fn update_pos_vel(&self) {
		self.pos.bind_image_unit(0, READ_WRITE);
		self.vel.bind_image_unit(1, READ_WRITE);
		self.acc.bind_image_unit(2, READ_ONLY);
		self.exec(self.p_verlet)
	}

	fn apply_mouse(&self) {
		self.pos.bind_image_unit(0, READ_WRITE);
		self.exec(self.p_mouse)
	}

	fn update_photon(&self) {
		self.photon.bind_image_unit(0, READ_WRITE);
		self.exec(self.p_decay);

		self.pos.bind_image_unit(0, READ_WRITE); // TODO
		self.photon.bind_image_unit(1, READ_WRITE);
		self.exec(self.p_photon);
	}

	fn draw(&self, _w: &Window) {
		glClearColor(0.5, 0.5, 0.5, 1.0);
		glClear(gl::COLOR_BUFFER_BIT);

		self.p_render.use_program();
		self.vao.bind();
		self.photon.bind_texture_unit(3);

		glDrawArrays(gl::TRIANGLE_STRIP, 0, 4);
	}

	fn exec(&self, p: Program) {
		let xy = self.pos.size();
		p.compute_and_sync(uvec3(xy.0, xy.1, 1))
	}

	fn on_cursor_moved(&self, position: (f64, f64)) {
		let (w, h) = (self.pos.size().0, self.pos.size().1);
		let (x, y) = ((position.0) as i32, (position.1) as i32);
		if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
			self.p_mouse.set2i("mouse_pos", x, y);
		}
	}

	fn on_mouse_input(&self, button: MouseButton, state: ElementState) {
		let sign = match button {
			glutin::event::MouseButton::Right => -1.0,
			_ => 1.0,
		};
		let pow = match state {
			ElementState::Pressed => MAX_POW,
			ElementState::Released => MIN_POW,
		};
		self.p_mouse.set1f("mouse_pow", sign * pow);
	}

	fn on_redraw_requested(&mut self, win: &Window) {
		self.draw(&win);
		win.swap_buffers().unwrap();
		self.steps(self.time_steps_per_draw);
		self.frames.set(self.frames.get() + 1);
		let secs = self.start.elapsed().as_secs_f32();
		let fps = self.frames.get() as f32 / secs;
		dbg!(fps);
	}

	fn on_user_event(&self, win: &Window) {
		win.window().request_redraw()
	}

	fn on_cursor_entered(&self) {
		self.p_mouse.set1f("mouse_pow", MIN_POW);
	}

	fn on_cursor_left(&self) {
		self.p_mouse.set1f("mouse_pow", 0.0);
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
				WindowEvent::CursorEntered { .. } => s.on_cursor_entered(),
				WindowEvent::CursorLeft { .. } => s.on_cursor_left(),
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_ => (),
			},
			_ => (),
		}
	});
}
