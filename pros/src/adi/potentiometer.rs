use crate::adi::{
    AdiError,
    AdiSlot
};

use pros_sys::PROS_ERR;

use crate::error::bail_on;

pub struct AdiPotentiometer {
    port: u8,
    reference: i32
}

impl AdiPotentiometer {
    /// Create an AdiPotentiometer without checking if it is valid.
    ///
    /// # Safety
    ///
    /// The port must be above 0 and below [`pros_sys::NUM_ADI_PORTS`].
    pub unsafe fn new_unchecked(port: AdiSlot) -> Self {
        Self {
            port: port as u8,
            reference: pros_sys::adi_potentiometer_init(port as u8)
        }
    }

    /// Create an AdiPotentiometer, throwing an error if the port is invalid.
    /// 
    /// # Panics
    /// 
    /// Panics if the port is greater than [`pros_sys::NUM_ADI_PORTS`].
    pub unsafe fn new(port: AdiSlot) -> Self {
        if {port as u8} < 1 || {port as u8} > {pros_sys::NUM_ADI_PORTS as u8} {
            panic!("Invalid ADI port");
        }
        Self {
            port: port as u8,
            reference: pros_sys::adi_potentiometer_init(port as u8)
        }
    }

    /// Create an AdiPotentiometer, returning err `AdiError::InvalidPort` if the port is invalid.
    pub unsafe fn try_new(port: AdiSlot) -> Result<Self, AdiError> {
        if {port as u8} < 1 || {port as u8} > {pros_sys::NUM_ADI_PORTS as u8} {
            return Err(AdiError::InvalidPort);
        }
        Ok(Self {
            port: port as u8,
            reference: pros_sys::adi_potentiometer_init(port as u8)
        })
    }

    /// Gets the current potentiometer angle in tenths of a degree.
    ///
    /// The original potentiometer rotates 250 degrees thus returning an angle between 0-250 degrees. Potentiometer V2 rotates 330 degrees thus returning an angle between 0-330 degrees. This function uses the following values of errno when an error state is reached:
    pub fn angle(&self) -> Result<f64, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR.into(), pros_sys::adi_potentiometer_get_angle(self.reference)) })
    }
}