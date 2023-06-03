use std::io;

use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use crate::protocol::{self, Response};

#[derive(Clone, Debug)]
pub struct Sender<W: io::Write> {
	w: W,
}
pub trait IntoSender: Sized + io::Write {
	fn into_sender(self) -> Sender<Self>;
}
impl<T: io::Write> IntoSender for T {
	fn into_sender(self) -> Sender<Self> {
		Sender { w: self }
	}
}
impl<W: io::Write> Sender<W> {
	fn open_in(&mut self, id: u16) -> crate::Result<()> {
		let msg = protocol::Message::OpenIn { id };

		let mut buf = vec![];
		msg.serialize(&mut Serializer::new(&mut buf))?;

		let len = buf.len() as u16;
		let len_bytes = u16::to_le_bytes(len);

		self.w.write_all(&len_bytes)?;
		self.w.write_all(&buf)?;

		Ok(())
	}
	fn open_out(&mut self, id: u16) -> crate::Result<()> {
		let msg = protocol::Message::OpenOut { id };

		let mut buf = vec![];
		msg.serialize(&mut Serializer::new(&mut buf))?;

		let len = buf.len() as u16;
		let len_bytes = u16::to_le_bytes(len);

		self.w.write_all(&len_bytes)?;
		self.w.write_all(&buf)?;

		Ok(())
	}
	fn request_in(&mut self, id: u16) -> crate::Result<()> {
		let msg = protocol::Message::RequestIn { id };

		let mut buf = vec![];
		msg.serialize(&mut Serializer::new(&mut buf))?;

		let len = buf.len() as u16;
		let len_bytes = u16::to_le_bytes(len);

		self.w.write_all(&len_bytes)?;
		self.w.write_all(&buf)?;

		Ok(())
	}
	fn set_out(&mut self, id: u16, val: crate::GpioValue) -> crate::Result<()> {
		let msg = protocol::Message::SetOut {
			id,
			val: val.into(),
		};

		let mut buf = vec![];
		msg.serialize(&mut Serializer::new(&mut buf))?;

		let len = buf.len() as u16;
		let len_bytes = u16::to_le_bytes(len);

		self.w.write_all(&len_bytes)?;
		self.w.write_all(&buf)?;

		Ok(())
	}

	fn check_for_resp<R: io::Read>(&mut self, mut r: R) -> crate::Result<protocol::Response> {
		let mut len_bytes = [0x00; 2];
		r.read_exact(&mut len_bytes)?;
		let len = u16::from_le_bytes(len_bytes);

		let mut buf = vec![0x00; len as usize];
		r.read_exact(&mut buf)?;

		let mut de = Deserializer::new(&buf[..]);
		Deserialize::deserialize(&mut de).map_err(|_| crate::Error::Protocol())?
	}
}
