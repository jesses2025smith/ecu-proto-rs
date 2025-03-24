//! Commons of Service 84


use std::ops::{BitAnd, BitXorAssign};
use bitfield_struct::bitfield;
use crate::{Iso14229Error, utils};


/// Table 490 — Definition of Administrative Parameter
///
/// ### Repr: `u16`
/// | Field                                  | Size (bits) |
/// |----------------------------------------|-------------|
/// | Message is request message             | 1           |
/// | ISO Reserved                           | 2           |
/// | A pre-established key is used          | 1           |
/// | Message is encrypted                   | 1           |
/// | Message is signed                      | 1           |
/// | Signature on the response is requested | 1           |
/// | ISO reserved                           | 4           |
/// | ISO reserved                           | 5           |
#[bitfield(u16, order = Msb)]
pub struct AdministrativeParameter {
    pub request: bool,
    #[bits(2)]
    __: u8,
    pub pre_established: bool,
    pub encrypted: bool,
    pub signed: bool,
    pub signature_on_response: bool,
    #[bits(4)]
    __: u8,
    #[bits(5)]
    __: u8,
}

impl Into<Vec<u8>> for AdministrativeParameter {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl AdministrativeParameter {

    #[inline]
    pub const fn is_request(&self) -> bool {
        self.request()
    }

    #[inline]
    pub fn request_set(&mut self, value: bool) -> &mut Self {
        self.set_request(value);
        self
    }

    #[inline]
    pub const fn is_pre_established(&self) -> bool {
        self.pre_established()
    }

    #[inline]
    pub fn pre_established_set(&mut self, value: bool) -> &mut Self {
        self.set_pre_established(value);
        self
    }

    #[inline]
    pub const fn is_encrypted(&self) -> bool {
        self.encrypted()
    }

    #[inline]
    pub fn encrypted_set(&mut self, value: bool) -> &mut Self {
        self.set_encrypted(value);
        self
    }

    #[inline]
    pub const fn is_signed(&self) -> bool {
        self.signed()
    }

    #[inline]
    pub fn signed_set(&mut self, value: bool) -> &mut Self {
        self.set_signed(value);
        self
    }

    #[inline]
    pub const fn is_signature_on_response(&self) -> bool {
        self.signature_on_response()
    }

    #[inline]
    pub fn signature_on_response_set(&mut self, value: bool) -> &mut Self {
        self.set_signature_on_response(value);
        self
    }
}

/// Table 491 — Definition of Signature/Encryption calculation parameter
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SignatureEncryptionCalculation {
    VehicleManufacturerSpecific(u8),    // 00 to 7F
    SystemSupplier(u8),                 // 80 to 8F
}

impl TryFrom<u8> for SignatureEncryptionCalculation {
    type Error = Iso14229Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00..=0x7F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x80..=0x8F => Ok(Self::SystemSupplier(value)),
            v => Err(Iso14229Error::ReservedError(v.to_string())),
        }
    }
}

impl Into<u8> for SignatureEncryptionCalculation {
    #[inline]
    fn into(self) -> u8 {
        match self {
            SignatureEncryptionCalculation::VehicleManufacturerSpecific(v) |
            SignatureEncryptionCalculation::SystemSupplier(v) => v,
        }
    }
}

#[cfg(test)]
mod test_apar {
    use super::AdministrativeParameter;

    #[test]
    fn apar() -> anyhow::Result<()> {
        let mut value: AdministrativeParameter = Default::default();
        assert_eq!(value.is_request(), false);

        value.request_set(true);
        assert_eq!(value.is_request(), true);

        Ok(())
    }
}
