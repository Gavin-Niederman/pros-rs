use crate::adi::{
    AdiError,
    AdiSlot
};

use crate::error::bail_on;

use pros_sys::PROS_ERR;

pub struct AdiAnalogIn {
    port: u8,
}

impl AdiAnalogIn {
    pub fn new(slot: AdiSlot) -> Self {
        let port = slot as u8;
        Self { port }
    }

    pub fn calibrate(&mut self) -> Result<i32, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR, pros_sys::adi_analog_calibrate(self.port)) })
    }

    pub fn value(&self) -> Result<i32, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR, pros_sys::adi_analog_read(self.port)) })
    }

    pub fn value_calibrated(&self) -> Result<i32, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR, pros_sys::adi_analog_read_calibrated(self.port)) })
    }

    pub fn value_calibrated_hr(&self) -> Result<i32, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR, pros_sys::adi_analog_read_calibrated_HR(self.port)) })
    }
}

pub struct AdiAnalogOut {
    port: u8,
}

impl AdiAnalogOut {
    pub fn new(slot: AdiSlot) -> Self {
        let port = slot as u8;
        Self { port }
    }

    pub fn set_value(&mut self, value: i32) -> Result<(), AdiError> {
        bail_on! {
            PROS_ERR,
            unsafe { pros_sys::adi_port_set_value(self.port, value) }
        }
    }
}