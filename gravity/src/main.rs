extern crate gl_img;
extern crate gl_safe;
extern crate gl_win;
extern crate image;
extern crate structopt;

use gl::*;
use gl_img::*;
use gl_obj::*;
use gl_safe::*;
use gl_win::*;
use glutin::event::ElementState;
use glutin::event::MouseButton;
use std::sync::Arc;
use std::time;
use structopt::StructOpt;

/// Gravity simulation.
#[derive(StructOpt)]
struct Args {
	/// Image width (pixels).
	#[structopt(short, long, default_value = "512")]
	width: u32,

	/// Image height, (pixels).
	#[structopt(short, long, default_value = "512")]
	height: u32,

	/// Number of particles.
	#[structopt(short, long, default_value = "16")]
	num_particles: u32,

	/// Verlet integration time step.
	#[structopt(long, default_value = "0.6")]
	dt: f32,
}

fn main() {
	let args = Args::from_args();

	// window
	let size = uvec2(args.width, args.height);
	let (win, ev) = init_gl_window(size.0, size.1, "gravity");

	let mut s = State::new(&args);

	let prog = s.p_verlet.into();

	{
		let block_index = glGetProgramResourceIndex(prog, gl::SHADER_STORAGE_BLOCK, "pos");
		println!("p_verlet pos index: {}", block_index);
		let binding_point_index = block_index;
		glShaderStorageBlockBinding(prog, block_index, binding_point_index);
		glBindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_point_index, s.pos.handle());
	}
	{
		let block_index = glGetProgramResourceIndex(prog, gl::SHADER_STORAGE_BLOCK, "vel");
		println!("p_verlet vel index: {}", block_index);
		let binding_point_index = block_index;
		glShaderStorageBlockBinding(prog, block_index, binding_point_index);
		glBindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_point_index, s.vel.handle());
	}

	s.exec(s.p_verlet);

	for p in s.pos.get_data() {
		println!("pos {:?}", p)
	}
	for p in s.pos.get_data() {
		println!("vel {:?}", p)
	}

	// s.p_accel //
	// 	.set1f("damping", args.damping);

	// s.p_verlet //
	// 	.set1f("dt", args.dt);

	// s.p_mouse //
	// 	.set1f("mouse_rad", args.mouse_radius);

	// s.p_photon //
	// 	.set1f("depth", args.depth)
	// 	.set1f("eta", args.refraction)
	// 	.set1f("dispersion", args.dispersion / 5.0); // div by 5 because we apply this 5 times (between red,yellow,green,cyan,blue,purple)
	// 										 //.set("light_dir", vec3(0.03, 0.01, -1.0).normalized())

	// s.p_render //
	// 	.set1f("water_refraction_depth", args.depth)
	// 	.set1f("water_refraction", args.refraction)
	// 	.set1f("reflection_height", args.sky_height)
	// 	.set1f("reflection_strength", args.reflection)
	// 	//.set1f("light_dir", light_dir)
	// 	.set1f("sun_strength", args.sun)
	// 	.set1f("photon_strength", args.caustics)
	// 	.set1f("ambient", args.ambient);

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
	background: vec3,
	pos: Buffer<vec4>,
	vel: Buffer<vec4>,
	acc: Buffer<vec4>,
	//p_accel: Program,
	p_verlet: Program,
	//p_mouse: Program,
	//p_normal: Program,
	//p_render: Program,
	//p_photon: Program,
	//p_decay: Program,
	//normal: Texture,
	//photon: Texture,
	//sky: Texture,
	//floor: Texture,
	//vao: VertexArray,
	//time_steps_per_draw: u32,
	//rand_seed: i32,
	//start: time::Instant,
	//frames: Cell<i32>,
}

impl State {
	fn new(args: &Args) -> Self {
		//let p_render = Program::new(&[
		//	//
		//	Shader::new_vert(include_str!("texture.vert")),
		//	Shader::new_frag(include_str!("water.frag")),
		//]);

		let FLAGS = 0;

		Self {
			background: vec3(0.5, 0.5, 0.5),
			pos: Buffer::new(&Self::init_pos(&args), FLAGS),
			vel: Buffer::new(&Self::init_pos(&args), FLAGS),
			acc: Buffer::new(&Self::init_pos(&args), FLAGS),
			p_verlet: Self::compute_prog(include_str!("verlet.glsl")),
			//	p_accel: Self::compute_prog(include_str!("accel.glsl")),
			//	p_mouse: Self::compute_prog(include_str!("apply_mouse.glsl")),
			//	p_normal: Self::compute_prog(include_str!("normal.glsl")),
			//	p_decay: Self::compute_prog(include_str!("udecay.glsl")),
			//	p_photon: Self::compute_prog(include_str!("photon.glsl")),
			//	p_render,
			//	pos: Texture::new2d(R32F, size),
			//	vel: Texture::new2d(R32F, size),
			//	acc: Texture::new2d(R32F, size),
			//	normal: Texture::new2d(gl::RGBA32F, size),
			//	photon: Texture::new2d(gl::RGBA8UI, size).filter_nearest(),
			//	sky: load_image(sky).filter_linear().clamp_to_edge(), // TODO !!
			//	floor: load_image(floor).filter_linear().mirrored_repeat(),
			//	vao: Self::vao(p_render),
			//	time_steps_per_draw: 6,
			//	rand_seed: 0,
			//	start: time::Instant::now(),
			//	frames: Cell::new(0),
		}
	}

	fn init_pos(args: &Args) -> Vec<vec4> {
		let n_particles = args.num_particles as usize;
		let mut pos = Vec::<vec4>::with_capacity(n_particles);
		for _ in 0..n_particles {
			pos.push(vec4(1.0, 2.0, 3.0, 4.0));
		}
		pos
	}

	fn step(&mut self) {
		self.update_pos_vel();

		//for _ in 0..n {
		//	self.update_acc();
		//	self.update_pos_vel();
		//	self.apply_mouse();
		//}
		//self.update_normal();
		//self.rand_seed += 1;
		//self.p_photon.set1i("rand_seed", self.rand_seed);
		//self.update_photon();
	}

	fn update_acc(&self) {
		//self.pos.bind_image_unit(0, READ_ONLY);
		//self.vel.bind_image_unit(1, READ_ONLY);
		//self.acc.bind_image_unit(2, WRITE_ONLY);
		//self.exec(self.p_accel)
	}

	fn update_pos_vel(&self) {
		//self.p_verlet.uniform1i(0, self.pos.handle() as i32);
		//self.pos.bind_image_unit(0, READ_WRITE);
		//self.vel.bind_image_unit(1, READ_WRITE);
		//self.acc.bind_image_unit(2, READ_ONLY);
		self.exec(self.p_verlet)
	}

	fn apply_mouse(&self) {
		//self.pos.bind_image_unit(0, READ_WRITE);
		//self.exec(self.p_mouse)
	}

	//fn update_photon(&self) {
	//	self.photon.bind_image_unit(0, READ_WRITE);
	//	self.exec(self.p_decay);

	//	self.normal.bind_image_unit(0, READ_ONLY);
	//	self.photon.bind_image_unit(1, READ_WRITE);
	//	self.exec(self.p_photon);
	//}

	fn draw(&self, _w: &Window) {
		let bg = self.background;
		glClearColor(bg.0, bg.1, bg.2, 1.0);
		glClear(gl::COLOR_BUFFER_BIT);

		// self.p_render.use_program();
		// self.vao.bind();
		// self.normal.bind_texture_unit(0);
		// self.sky.bind_texture_unit(1);
		// self.floor.bind_texture_unit(2);
		// self.photon.bind_texture_unit(3);

		// glDrawArrays(gl::TRIANGLE_STRIP, 0, 4);
	}

	fn exec(&self, p: Program) {
		p.compute_and_sync(uvec3(self.pos.len() as u32, 1, 1))
	}

	fn on_cursor_moved(&self, position: (f64, f64)) {
		//let (w, h) = (self.pos.size().0, self.pos.size().1);
		//let (x, y) = ((position.0) as i32, (position.1) as i32);
		//if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
		//	self.p_mouse.set2i("mouse_pos", x, y);
		//}
	}

	fn on_mouse_input(&self, button: MouseButton, state: ElementState) {
		//let sign = match button {
		//	glutin::event::MouseButton::Right => -1.0,
		//	_ => 1.0,
		//};
		//let pow = match state {
		//	ElementState::Pressed => MAX_POW,
		//	ElementState::Released => MIN_POW,
		//};
		//self.p_mouse.set1f("mouse_pow", sign * pow);
	}

	fn on_redraw_requested(&self, win: &Window) {
		self.draw(&win);
		win.swap_buffers().unwrap();
		// self.steps(self.time_steps_per_draw);
		// self.frames.set(self.frames.get() + 1);
		// let secs = self.start.elapsed().as_secs_f32();
		// let fps = self.frames.get() as f32 / secs;
		// dbg!(fps);
	}

	fn on_user_event(&self, win: &Window) {
		win.window().request_redraw()
	}

	fn on_cursor_entered(&self) {
		//self.p_mouse.set1f("mouse_pow", MIN_POW);
	}

	fn on_cursor_left(&self) {
		//self.p_mouse.set1f("mouse_pow", 0.0);
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
