#[cfg(feature = "native")]
pub mod native;
#[cfg(feature = "receive")]
pub mod receive;
#[cfg(feature = "send")]
pub mod send;

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("gpio error")]
	GPIO(),
	#[error("gpio id out of bounds")]
	IDOutOfBounds(),
	#[error("io error")]
	IO(io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioValue {
	Low,
	High,
}
impl From<gpio::GpioValue> for GpioValue {
	fn from(value: gpio::GpioValue) -> Self {
		match value {
			gpio::GpioValue::Low => GpioValue::Low,
			gpio::GpioValue::High => GpioValue::High,
		}
	}
}
impl From<bool> for GpioValue {
	fn from(value: bool) -> Self {
		match value {
			false => GpioValue::Low,
			true => GpioValue::High,
		}
	}
}

pub trait GpioIn {
	type Error;
	fn read_value(&mut self) -> crate::Result<GpioValue>;
}
impl<T: gpio::GpioIn> GpioIn for T {
	type Error = <Self as gpio::GpioIn>::Error;
	fn read_value(&mut self) -> crate::Result<GpioValue> {
		(self as &mut T)
			.read_value()
			.map_err(|_| Error::GPIO())
			.map(|val| val.into())
	}
}

pub trait GpioOut {
	type Error;
	fn set_low(&mut self) -> crate::Result<()>;
	fn set_high(&mut self) -> crate::Result<()>;

	fn set<T: Into<GpioValue> + Copy>(&mut self, value: T) -> crate::Result<()> {
		match value.into() {
			GpioValue::Low => self.set_low(),
			GpioValue::High => self.set_high(),
		}
	}
}
impl<T: gpio::GpioOut> GpioOut for T {
	type Error = <Self as gpio::GpioOut>::Error;
	fn set_low(&mut self) -> crate::Result<()> {
		(self as &mut T).set_low().map_err(|_| Error::GPIO())
	}
	fn set_high(&mut self) -> crate::Result<()> {
		(self as &mut T).set_high().map_err(|_| Error::GPIO())
	}
}

pub trait Interface {
	type In: GpioIn;
	type Out: GpioOut;

	fn open_in(id: u16) -> crate::Result<Self::In>;
	fn open_out(id: u16) -> crate::Result<Self::Out>;
}
