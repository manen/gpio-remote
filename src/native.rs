#[derive(Clone, Debug)]
pub struct Native {}
impl crate::Interface for Native {
	type In = NativeIn;
	type Out = NativeOut;

	fn open_in(&mut self, id: u16) -> crate::Result<Self::In> {
		gpio::sysfs::SysFsGpioInput::open(id)
			.map(|native| NativeIn { native })
			.map_err(|err| crate::Error::IO(err))
	}
	fn open_out(&mut self, id: u16) -> crate::Result<Self::Out> {
		gpio::sysfs::SysFsGpioOutput::open(id)
			.map(|native| NativeOut { native })
			.map_err(|err| crate::Error::IO(err))
	}
}

#[derive(Debug)]
pub struct NativeIn {
	native: gpio::sysfs::SysFsGpioInput,
}
impl crate::GpioIn for NativeIn {
	type Error = <gpio::sysfs::SysFsGpioInput as gpio::GpioIn>::Error;
	#[inline(always)]
	fn read_value(&mut self) -> crate::Result<crate::GpioValue> {
		self.native.read_value()
	}
}

#[derive(Debug)]
pub struct NativeOut {
	native: gpio::sysfs::SysFsGpioOutput,
}
impl crate::GpioOut for NativeOut {
	type Error = <gpio::sysfs::SysFsGpioOutput as gpio::GpioOut>::Error;
	#[inline(always)]
	fn set_high(&mut self) -> crate::Result<()> {
		self.native.set_high()
	}
	#[inline(always)]
	fn set_low(&mut self) -> crate::Result<()> {
		self.native.set_low()
	}
}
