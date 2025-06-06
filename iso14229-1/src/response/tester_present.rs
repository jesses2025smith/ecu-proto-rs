//! response of Service 3E


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{utils, Iso14229Error, response::{Code, Response, SubFunction}, Service, TesterPresentType, Configuration, ResponseData};

lazy_static!(
    pub static ref TESTER_PRESENT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
    ]);
);

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TesterPresent {
    pub data: Vec<u8>,  // should emtpy
}

impl ResponseData for TesterPresent {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let _ = TesterPresentType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::TesterPresent,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            },
            None => Err(Iso14229Error::SubFunctionError(Service::TesterPresent)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::TesterPresent
            || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service))
        }

        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
