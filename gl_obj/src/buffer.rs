use super::*;
use core::any::{self, TypeId};
use gl_safe::*;
use std::marker::PhantomData;
use std::mem::size_of;

pub struct Buffer<T: Sized + Copy + 'static> {
	handle: GLuint,
	len: usize,
	_type: PhantomData<T>,
	//stride: u32,
	//typeid: TypeId,
	//typename: &'static str,
}

impl<T> Buffer<T>
where
	T: Sized + Copy + 'static,
{
	pub fn new(data: &[T], flags: GLbitfield) -> Self {
		Self::create().storage(data, flags)
	}

	/// Create a buffer object.
	/// http://docs.gl/gl4/glCreateBuffers
	pub fn create() -> Self {
		Self {
			handle: glCreateBuffer(),
			len: 0,
			_type: PhantomData,
		}
	}

	/// Creates and initializes a buffer object's immutable data store.
	/// http://docs.gl/gl4/glBufferStorage
	pub fn storage(self, data: &[T], flags: GLbitfield) -> Self {
		glNamedBufferStorage(self.handle, data, flags);
		Self { len: data.len(), ..self }
	}

	pub fn len(&self) -> usize {
		self.len
	}

	/// Copy the buffer's contents into `data`,
	/// which must match in size.
	pub fn copy_data(&self, data: &mut [T]) {
		if data.len() != self.len() {
			panic!("Buffer::get_data: size mismatch: buffer len {} != argument len {}", self.len(), data.len())
		}
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, data)
	}

	/// Returns a copy the buffer's contents.
	pub fn get_data(&self) -> Vec<T> {
		let mut data = Vec::<T>::with_capacity(self.len());
		unsafe { data.set_len(self.len()) };
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, &mut data);
		data
	}

	//pub fn stride(&self) -> i32 {
	//	self.stride as i32
	//}

	//pub fn bytes(&self) -> usize {
	//	self.len * (self.stride as usize)
	//}
}

impl<T> Into<GLuint> for Buffer<T>
where
	T: Sized + Copy + 'static,
{
	fn into(self) -> GLuint {
		self.handle
	}
}
