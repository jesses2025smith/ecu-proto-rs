use crate::enum_to_vec;
use crate::error::Error;
use crate::service::{Configuration, Placeholder, RequestData};
use crate::utils;

enum_to_vec!(
    /// Table C.10 — transmissionMode parameter definitions
    pub enum TransmissionMode {
        SendAtSlowRate = 0x01,
        SendAtMediumRate = 0x02,
        SendAtFastRate = 0x03,
        StopSending = 0x04,
    }, u8, Error, InvalidParam
);

pub struct ReadByPeriodIdData {
    mode: TransmissionMode,
    did: Vec<u8>,
}

impl ReadByPeriodIdData {
    pub fn new(
        mode: TransmissionMode,
        did: Vec<u8>
    ) -> Result<Self, Error> {
        match mode {
            TransmissionMode::SendAtSlowRate |
            TransmissionMode::SendAtMediumRate |
            TransmissionMode::SendAtFastRate => {
                if did.is_empty() {
                    return Err(Error::InvalidParam("empty period_id".to_string()));
                }

                Ok(())
            },
            TransmissionMode::StopSending => Ok(()),
        }?;

        Ok(Self { mode, did })
    }

    #[inline]
    pub fn transmission_mode(&self) -> TransmissionMode {
        self.mode.clone()
    }

    #[inline]
    pub fn period_did(&self) -> &Vec<u8> {
        &self.did
    }
}

impl<'a> TryFrom<&'a [u8]> for ReadByPeriodIdData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 1, false)?;

        let mut offset = 0;
        let mode = TransmissionMode::try_from(data[offset])?;
        offset += 1;

        let did = data[offset..].to_vec();

        Self::new(mode, did)
    }
}

impl Into<Vec<u8>> for ReadByPeriodIdData {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.mode.into(), ];
        result.append(&mut self.did);

        result
    }
}

impl RequestData for ReadByPeriodIdData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}
