use std::io;

use rmp_serde::Deserializer;
use serde::Deserialize;

use crate::{protocol, GpioIn, GpioOut};

#[derive(Clone, Copy, Debug)]
pub struct GPIOBuf<T> {
	buf: [T; 40],
}
impl<T> Default for GPIOBuf<Option<T>> {
	fn default() -> Self {
		let mut default: Self = unsafe { std::mem::zeroed() };
		for i in 0..39 {
			default.buf[i] = None;
		}
		default
	}
}
impl<T> GPIOBuf<T> {
	fn index(&mut self, id: u16) -> crate::Result<&mut T> {
		Ok(&mut self.buf[id as usize - 1]) // TODO bounds checking ig
	}
	fn index_set(&mut self, id: u16, val: T) -> crate::Result<()> {
		self.buf[id as usize - 1] = val;
		Ok(())
	}
}

#[derive(Clone, Debug)]
pub struct Receiver<I: crate::Interface> {
	buf_in: GPIOBuf<Option<I::In>>,
	buf_out: GPIOBuf<Option<I::Out>>,
	resp_buf: Vec<protocol::Response>,
	i: I,
}
trait IntoReceiver: Sized + crate::Interface {
	fn into_receiver(self) -> Receiver<Self>;
}
impl<T: crate::Interface> IntoReceiver for T {
	fn into_receiver(self) -> Receiver<Self> {
		Receiver::new(self)
	}
}
impl<I: crate::Interface> Receiver<I> {
	pub fn new(i: I) -> Self {
		Receiver {
			buf_in: Default::default(),
			buf_out: Default::default(),
			resp_buf: vec![],
			i,
		}
	}

	pub fn execute<R: io::Read>(&mut self, mut r: R) -> crate::Result<()> {
		let mut len_bytes = [0x00; 2];
		r.read_exact(&mut len_bytes)?;
		let len = u16::from_le_bytes(len_bytes);

		let mut buf = vec![0x00; len as usize];
		r.read_exact(&mut buf)?;

		let mut de = Deserializer::new(&buf[..]);
		let msg: protocol::Message =
			Deserialize::deserialize(&mut de).map_err(|_| crate::Error::Protocol())?;

		match msg {
			protocol::Message::OpenIn { id } => {
				if let Some(_) = self.buf_in.index(id)? {
					log::info!("gpio remote: requested in {id}, but it already exists in cache");
				} else {
					self.buf_in.index_set(id, Some(self.i.open_in(id)?))?;
				}
			}
			protocol::Message::OpenOut { id } => {
				if let Some(_) = self.buf_out.index(id)? {
					log::info!("gpio remote: requested out {id}, but it already exists in cache");
				} else {
					self.buf_out.index_set(id, Some(self.i.open_out(id)?))?;
				}
			}
			protocol::Message::RequestIn { id } => {
				if let Some(a) = self.buf_in.index(id)? {
					self.resp_buf.push(protocol::Response::InValue {
						id,
						val: a.read_value()?.into(),
					});
				} else {
					log::warn!(
						"gpio remote: requested value of in {id}, but it doesn't exist in cache"
					);
				}
			}
			protocol::Message::SetOut { id, val } => {
				if let Some(a) = self.buf_out.index(id)? {
					a.set(val)?;
				} else {
					log::warn!(
						"gpio remote: attempted to set out {id}, but it doesn't exist in cache"
					);
				}
			}
		}

		Ok(())
	}
}
