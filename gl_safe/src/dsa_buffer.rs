use super::check;
use super::*;
use std::mem;

/// Generate a (single) buffer object name.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml
/// TODO: remove in favour of Create.
//#[allow(non_snake_case)]
//pub fn glGenBuffer() -> GLuint {
//	let mut buffers = 0;
//	unsafe { gl::GenBuffers(1, &mut buffers) }
//	check::gl_error();
//	buffers
//}

/// Create a buffer object.
/// http://docs.gl/gl4/glCreateBuffers
#[allow(non_snake_case)]
pub fn glCreateBuffer() -> GLuint {
	let mut result = 0;
	unsafe { gl::CreateBuffers(1, &mut result) };
	check::gl_error();
	result
}

/// Creates and initializes a buffer object's immutable data store.
/// http://docs.gl/gl4/glBufferStorage
#[allow(non_snake_case)]
pub fn glNamedBufferStorage<T>(buffer: GLuint, data: &[T], flags: GLbitfield)
where
	T: Sized + Copy + 'static,
{
	let bytes = data.len() * mem::size_of::<T>();
	unsafe { gl::NamedBufferStorage(buffer, bytes as isize, mem::transmute(&data[0]), flags) }
	check::gl_error();
}

/*
/// Creates and initializes a buffer object's data store.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
#[allow(non_snake_case)]
pub fn glBufferData<T>(target: GLenum, data: &[T], usage: GLenum)
where
	T: Sized + 'static,
{
	let size = (data.len() * mem::size_of::<T>()) as GLsizeiptr;
	let data = unsafe { mem::transmute(&data[0]) };
	unsafe { gl::BufferData(target, size, data, usage) };
	check::gl_error();
}
*/

/*
/// Create a buffer object.
/// http://docs.gl/gl4/glCreateBuffers
#[allow(non_snake_case)]
pub fn glCreateBuffer() -> GLuint {
	let mut result = 0;
	unsafe { gl::CreateBuffers(1, &mut result) };
	check::gl_error();
	result
}
*/

/*
/// Simultaneously specify storage for all levels of a one-dimensional texture.
/// http://docs.gl/gl4/glTexStorage1D
#[allow(non_snake_case)]
pub fn glTextureStorage1D(texture: GLuint, levels: i32, internalformat: GLenum, width: i32) {
	unsafe { gl::TextureStorage1D(texture, levels, internalformat, width) };
	check::gl_error()
}

/// Simultaneously specify storage for all levels of a two-dimensional or one-dimensional array texture.
/// http://docs.gl/gl4/glTexStorage2D
#[allow(non_snake_case)]
pub fn glTextureStorage2D(texture: GLuint, levels: i32, internalformat: GLenum, width: i32, height: i32) {
	unsafe { gl::TextureStorage2D(texture, levels, internalformat, width, height) };
	check::gl_error()
}

/// Simultaneously specify storage for all levels of a three-dimensional, two-dimensional array or cube-map array texture.
/// http://docs.gl/gl4/glTexStorage3D
#[allow(non_snake_case)]
pub fn glTextureStorage3D(texture: GLuint, levels: i32, internalformat: GLenum, width: i32, height: i32, depth: i32) {
	unsafe { gl::TextureStorage3D(texture, levels, internalformat, width, height, depth) };
	check::gl_error()
}

/// Specify a one-dimensional texture subimage.
/// http://docs.gl/gl4/glTexSubImage1D
#[allow(non_snake_case)]
pub fn glTextureSubImage1D<T>(texture: GLuint, level: i32, xoffset: i32, width: i32, format: GLenum, typ: GLenum, pixels: &[T])
where
	T: Sized + Copy + 'static,
{
	check::image_size(&[width], format, typ, pixels);
	unsafe { gl::TextureSubImage1D(texture, level, xoffset, width, format, typ, mem::transmute(&pixels[0])) };
	check::gl_error()
}

/// Specify a two-dimensional texture subimage.
/// http://docs.gl/gl4/glTexSubImage2D
#[allow(non_snake_case)]
pub fn glTextureSubImage2D<T>(texture: GLuint, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: GLenum, typ: GLenum, pixels: &[T])
where
	T: Sized + Copy + 'static,
{
	check::image_size(&[width, height], format, typ, pixels);
	unsafe { gl::TextureSubImage2D(texture, level, xoffset, yoffset, width, height, format, typ, mem::transmute(&pixels[0])) };
	check::gl_error()
}

/// Specify a three-dimensional texture subimage.
/// http://docs.gl/gl4/glTexSubImage3D
#[allow(non_snake_case)]
pub fn glTextureSubImage3D<T>(texture: GLuint, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: GLenum, typ: GLenum, pixels: &[T])
where
	T: Sized + Copy + 'static,
{
	check::image_size(&[width, height, depth], format, typ, pixels);
	unsafe { gl::TextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, typ, mem::transmute(&pixels[0])) };
	check::gl_error()
}

/// Set texture parameters.
/// http://docs.gl/gl4/glTexParameter
#[allow(non_snake_case)]
pub fn glTextureParameterf(texture: GLuint, pname: GLenum, param: f32) {
	unsafe { gl::TextureParameterf(texture, pname, param) };
	check::gl_error()
}

/// Set texture parameters.
/// http://docs.gl/gl4/glTexParameter
#[allow(non_snake_case)]
pub fn glTextureParameterfv(texture: GLuint, pname: GLenum, param: &[f32]) {
	unsafe { gl::TextureParameterfv(texture, pname, &param[0]) };
	check::gl_error()
}

/// Set texture parameters.
/// http://docs.gl/gl4/glTexParameter
#[allow(non_snake_case)]
pub fn glTextureParameteri(texture: GLuint, pname: GLenum, param: i32) {
	unsafe { gl::TextureParameteri(texture, pname, param) };
	check::gl_error()
}

/// Set texture parameters.
/// http://docs.gl/gl4/glTexParameter
#[allow(non_snake_case)]
pub fn glTextureParameteriv(texture: GLuint, pname: GLenum, param: &[i32]) {
	unsafe { gl::TextureParameteriv(texture, pname, &param[0]) };
	check::gl_error()
}

// //pub fn glTextureParameterIiv (texture: GLuint, pname: GLenum, const int *params){}
// //pub fn glTextureParameterIuiv(texture: GLuint, pname: GLenum, const uint *params){}

/// Bind an existing texture object to the specified texture unit.
/// http://docs.gl/gl4/glBindTextureUnit
#[allow(non_snake_case)]
pub fn glBindTextureUnit(unit: GLuint, texture: GLuint) {
	unsafe { gl::BindTextureUnit(unit, texture) };
	check::gl_error()
}
*/
