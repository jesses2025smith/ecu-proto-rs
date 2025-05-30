//! request of Service 2C

use rsutil::types::ByteOrder;
use crate::{DynamicallyDID, DefinitionType, DynamicallyMemAddr, Iso14229Error, request::{Request, SubFunction}, RequestData, Configuration, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DynamicallyDefineDID {
    DefineByIdentifier {
        did: DynamicallyDID,
        source: DynamicallyMemAddr,
        others: Vec<DynamicallyMemAddr>
    },
    DefineByMemoryAddress {
        did: DynamicallyDID,
        memory: (u128, u128),           // (mem_addr, mem_size),
        others: Vec<(u128, u128)>,      // at least one
    },
    ClearDynamicallyDefinedDataIdentifier(Option<DynamicallyDID>),
}

impl RequestData for DynamicallyDefineDID {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                let data_len = data.len();
                match DefinitionType::try_from(sub_func)? {
                    DefinitionType::DefineByIdentifier => utils::data_length_check(data_len, 6, false)?,
                    DefinitionType::DefineByMemoryAddress => utils::data_length_check(data_len, 4, false)?,
                    DefinitionType::ClearDynamicallyDefinedDataIdentifier => match data_len {
                        0 | 2 => {},
                        _ => return Err(Iso14229Error::InvalidDataLength { expect: 0, actual: data_len }),
                    }
                }

                Ok(Request {
                    service: Service::DynamicalDefineDID,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec()
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::DynamicalDefineDID)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service;
        if service != Service::DynamicalDefineDID
            || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: DefinitionType = request.sub_function().unwrap().function()?;
        let data = &request.data;
        let data_len = data.len();
        let mut offset = 0;
        match sub_func {
            DefinitionType::DefineByIdentifier => {
                let did = DynamicallyDID::try_from(
                    u16::from_be_bytes([data[offset], data[offset + 1]])
                )?;
                offset += 2;
                let source = DynamicallyMemAddr::try_from(&data[offset..])?;
                offset += 4;

                let mut others = Vec::new();
                while data_len > offset {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    others.push( DynamicallyMemAddr::try_from(&data[offset..])?);
                    offset += 4;
                }

                Ok(Self::DefineByIdentifier { did, source, others })
            }
            DefinitionType::DefineByMemoryAddress => {
                let did = DynamicallyDID::try_from(
                    u16::from_be_bytes([data[offset], data[offset + 1]])
                )?;
                offset += 2;

                let alfi = data[offset];
                offset += 1;
                let mem_addr_len = (alfi & 0x0F) as usize;
                let mem_size_len = ((alfi & 0xF0) >> 4) as usize;
                utils::data_length_check(data_len, offset + mem_addr_len + mem_size_len, false)?;

                let mem_addr = utils::slice_to_u128(&data[offset..offset + mem_addr_len], ByteOrder::Big);
                offset += mem_addr_len;
                let mem_size = utils::slice_to_u128(&data[offset..offset + mem_size_len], ByteOrder::Big);
                offset += mem_size_len;

                let mut others = Vec::new();
                while data_len > offset {
                    utils::data_length_check(data_len, offset + mem_addr_len + mem_size_len, false)?;

                    let mem_addr = utils::slice_to_u128(&data[offset..offset + mem_addr_len], ByteOrder::Big);
                    offset += mem_addr_len;
                    let mem_size = utils::slice_to_u128(&data[offset..offset + mem_size_len], ByteOrder::Big);
                    offset += mem_size_len;
                    others.push((mem_addr, mem_size));
                }

                Ok(Self::DefineByMemoryAddress { did, memory: (mem_addr, mem_size), others })
            }
            DefinitionType::ClearDynamicallyDefinedDataIdentifier => {
                let dyn_did = match data_len - offset {
                    0 => Ok(None),
                    2 => Ok(Some(DynamicallyDID::try_from(
                        u16::from_be_bytes([data[offset], data[offset + 1]])
                    )?)),
                    v => Err(Iso14229Error::InvalidDataLength { expect: 2, actual: v }),
                }?;

                Ok(Self::ClearDynamicallyDefinedDataIdentifier(dyn_did))
            }
        }
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        match self {
            Self::DefineByIdentifier {
                did,
                source,
                others,
            } => {
                result.append(&mut did.into());
                result.append(&mut source.into());
                others.into_iter()
                    .for_each(|v| result.append(&mut v.into()));
            },
            Self::DefineByMemoryAddress {
                did,
                // addr_len,
                // size_len,
                memory,
                others,
            } => {
                result.append(&mut did.into());

                let mut max_addr = memory.0;
                let mut max_size = memory.1;
                others.iter()
                    .for_each(|&v| {
                        let curr_addr = v.0;
                        let curr_size = v.1;
                        if curr_addr > max_addr {
                            max_addr = curr_addr;
                        }

                        if curr_size > max_size {
                            max_size = curr_size;
                        }
                    });

                let mem_addr_len = utils::length_of_u_type(max_addr);
                let mem_size_len = utils::length_of_u_type(max_size);
                result.push(((mem_size_len << 4) | mem_addr_len) as u8);

                let mut mem_addr = utils::u128_to_vec(memory.0, mem_addr_len, ByteOrder::Big);
                let mut mem_size = utils::u128_to_vec(memory.1, mem_size_len, ByteOrder::Big);
                result.append(&mut mem_addr);
                result.append(&mut mem_size);

                others.into_iter()
                    .for_each(|v| {
                        let mut mem_addr = utils::u128_to_vec(v.0, mem_addr_len, ByteOrder::Big);
                        let mut mem_size = utils::u128_to_vec(v.1, mem_size_len, ByteOrder::Big);
                        result.append(&mut mem_addr);
                        result.append(&mut mem_size);
                    });
            },
            Self::ClearDynamicallyDefinedDataIdentifier(did) => {
                if let Some(v) = did {
                    result.append(&mut v.into());
                }
            },
        }

        result
    }
}
