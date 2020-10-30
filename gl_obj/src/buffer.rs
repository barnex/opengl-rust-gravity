use super::*;
use core::any::{self, TypeId};
use gl_safe::*;
use std::mem::size_of;

#[derive(Clone)]
pub struct Buffer {
	handle: GLuint,
	len: usize,
	stride: u32,
	typeid: TypeId,
	typename: &'static str,
}

impl Buffer {
	/// Create a buffer object.
	/// http://docs.gl/gl4/glCreateBuffers
	pub fn create() -> Self {
		Self {
			handle: glCreateBuffer(),
			len: 0,
			stride: 0,
			typeid: TypeId::of::<()>(),
			typename: "",
		}
	}

	/// Creates and initializes a buffer object's immutable data store.
	/// http://docs.gl/gl4/glBufferStorage
	pub fn storage<T>(self, data: &[T], flags: GLbitfield) -> Self
	where
		T: Sized + Copy + 'static,
	{
		glNamedBufferStorage(self.handle, data, flags);
		Self {
			typeid: TypeId::of::<T>(),
			typename: std::any::type_name::<T>(),
			stride: size_of::<T>() as u32,
			len: data.len(),
			..self
		}
	}

	pub fn copy_data<T>(&self, data: &mut [T])
	where
		T: Sized + Copy + 'static,
	{
		self.check_type::<T>();
		if data.len() != self.len() {
			panic!("Buffer::get_data: size mismatch: buffer len {} != argument len {}", self.len(), data.len())
		}
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, data)
	}

	pub fn data<T>(&self) -> Vec<T>
	where
		T: Sized + Copy + 'static,
	{
		self.check_type::<T>();
		let mut data = Vec::<T>::with_capacity(self.len());
		unsafe { data.set_len(self.len()) };
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, &mut data);
		data
	}

	fn check_type<T>(&self)
	where
		T: Sized + Copy + 'static,
	{
		let have = TypeId::of::<T>();
		if have != self.typeid {
			panic!("Buffer::get_data: type mismatch: buffer type {:?} != argument type {:?}", self.typename, any::type_name::<T>())
		}
	}

	pub fn stride(&self) -> i32 {
		self.stride as i32
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn bytes(&self) -> usize {
		self.len * (self.stride as usize)
	}
}

impl Into<GLuint> for Buffer {
	fn into(self) -> GLuint {
		self.handle
	}
}
