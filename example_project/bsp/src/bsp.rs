use stm32f7xx_hal::{
    prelude::*,
    pac,
};
use embedded_hal::digital::v2::{OutputPin, InputPin};
use core::convert::Infallible;
extern crate alloc;
use alloc::boxed::Box;
pub struct Board {
    pub led1: Led,
    pub led2: Led,
    pub led3: Led,
    pub button: Button,
}
impl Board {
    pub fn new() -> Self {
        let p = pac::Peripherals::take().unwrap();
        let gpioc = p.GPIOC.split();
        let pin = gpioc.pc13.into_floating_input();
        let button = Button::new(pin);
        let gpiob = p.GPIOB.split();
        let pin = gpiob.pb0.into_push_pull_output();
        let led1 = Led::new(pin);
        let pin = gpiob.pb7.into_push_pull_output();
        let led2 = Led::new(pin);
        let pin = gpiob.pb14.into_push_pull_output();
        let led3 = Led::new(pin);
        Self { led1, led2, led3, button }

    }
}

pub struct Led {
    pin: Box< dyn OutputPin<Error = Infallible>>
}

impl Led {
    pub fn new(pin: impl OutputPin< Error = Infallible> + 'static) -> Self{
        Led { pin: Box::new(pin) }
    }
    pub fn on(&mut self){
        let _ = self.pin.set_high();
    }
    pub fn off(&mut self){
        let _ = self.pin.set_low();
    }
}

pub struct Button {
    pin: Box< dyn InputPin<Error = Infallible>>
}

impl Button {
    pub fn new(pin: impl InputPin< Error = Infallible> + 'static) -> Self {
        Self {pin: Box::new(pin)}
    }
    pub fn pressed(&self) -> bool {
        self.pin.is_high().expect("Infallible")
    }
}