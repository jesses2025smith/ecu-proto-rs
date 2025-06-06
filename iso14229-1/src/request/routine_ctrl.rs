//! request of Service 31


use crate::{Configuration, Iso14229Error, request::{Request, SubFunction}, RequestData, RoutineCtrlType, RoutineId, Service, utils};

#[derive(Debug, Clone)]
pub struct RoutineCtrl {
    pub routine_id: RoutineId,
    pub option_record: Vec<u8>,
}

impl RequestData for RoutineCtrl {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = RoutineCtrlType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request {
                    service: Service::RoutineCtrl,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            },
            None => Err(Iso14229Error::SubFunctionError(Service::RoutineCtrl)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RoutineCtrl
            || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service))
        }

        // let sub_func: RoutineCtrlType = request.sub_function().unwrap().function()?;

        let data = &request.data;
        let mut offset = 0;
        let routine_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let routine_id = RoutineId::from(routine_id);

        Ok(Self { routine_id, option_record: data[offset..].to_vec() })
    }
    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let routine_id: u16 = self.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        result.append(&mut self.option_record);

        result
    }
}
