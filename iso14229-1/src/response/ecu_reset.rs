//! response of Service 11

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, ECUResetType, Iso14229Error, response::{Code, Response, SubFunction}, ResponseData, Service, utils};

lazy_static!(
    pub static ref ECU_RESET_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::AuthenticationRequired,
    ]);
);

/// only sub-function is 0x04
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ECUReset {
    pub second: Option<u8>,
}

impl From<ECUReset> for Vec<u8> {
    fn from(val: ECUReset) -> Self {
        match val.second {
            Some(v) => vec![v, ],
            None => vec![],
        }
    }
}

impl ResponseData for ECUReset {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let data_len = data.len();
                match ECUResetType::try_from(sub_func)? {
                    ECUResetType::EnableRapidPowerShutDown => utils::data_length_check(data_len, 1, true)?,
                    _ => utils::data_length_check(data_len, 0, true)?,
                }

                Ok(Response {
                    service: Service::ECUReset,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            },
            None => Err(Iso14229Error::SubFunctionError(Service::ECUReset)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ECUReset
            || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service))
        }

        let sub_func: ECUResetType = response.sub_function().unwrap().function()?;
        let data = &response.data;
        let second = match sub_func {
            ECUResetType::EnableRapidPowerShutDown => Some(data[0]),
            _ => None,
        };

        Ok(ECUReset { second })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        match self.second {
            Some(v) => vec![v, ],
            None => vec![],
        }
    }
}
