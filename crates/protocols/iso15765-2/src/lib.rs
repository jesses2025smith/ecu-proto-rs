#![allow(deprecated)]

mod error;
pub use error::*;
mod constants;
pub use constants::*;

use std::fmt::{Display, Formatter};
use bitflags::bitflags;

bitflags! {
    /// ISO 15765-2 state.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IsoTpState: u8 {
        const Idle = 0b0000_0000;
        #[deprecated]
        const WaitSingle = 0b0000_0001;
        #[deprecated]
        const WaitFirst = 0b0000_0010;
        const WaitFlowCtrl = 0b0000_0100;
        #[deprecated]
        const WaitData = 0b0000_1000;
        const WaitBusy = 0b0001_0000;
        #[deprecated]
        const ResponsePending = 0b0010_0000;
        const Sending = 0b0100_0000;
        const Error = 0b1000_0000;
    }
}

impl Display for IsoTpState {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut idle = true;
        let mut first = true;
        if self.contains(IsoTpState::WaitSingle) {
            write!(f, "WaitSingle")?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::WaitFirst) {
            write!(f, "{}", format!("{}WaitFirst", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::WaitFlowCtrl) {
            write!(f, "{}", format!("{}WaitFlowCtrl", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::WaitData) {
            write!(f, "{}", format!("{}WaitData", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::WaitBusy) {
            write!(f, "{}", format!("{}WaitBusy", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::ResponsePending) {
            write!(f, "{}", format!("{}ResponsePending", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::Sending) {
            write!(f, "{}", format!("{}Sending", if first { "" } else { " | " }))?;
            idle = false;
            first = false;
        }
        if self.contains(IsoTpState::Error) {
            write!(f, "{}", format!("{}Error", if first { "" } else { " | " }))?;
            idle = false;
        }
        if idle {
            write!(f, "Idle")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum IsoTpEvent {
    Wait,
    FirstFrameReceived,
    DataReceived(Vec<u8>),
    ErrorOccurred(Iso15765Error),
}

pub trait IsoTpEventListener {
    fn buffer_data(&mut self) -> Option<IsoTpEvent>;
    fn clear_buffer(&mut self);
    fn on_iso_tp_event(&mut self, event: IsoTpEvent);
}

/// ISO 15765-2 frame type define.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameType {
    /// | - data length -| - N_PCI bytes - | - note - |
    ///
    /// | -     le 8   - | -  bit0(3~0) = length  - | - std2004 - |
    ///
    /// | -     gt 8    - | -  bit0(3~0) = 0; bit1(7~0) = length  - | - std2016 - |
    Single = 0x00,
    /// | - data length -| - N_PCI bytes - | - note - |
    ///
    /// | -  le 4095   - | - bit0(3~0) + bit1(7~0) = length - | - std2004 - |
    ///
    /// | -  gt 4095   - | - bit0(3~0) + bit1(7~0) = 0; byte2~5(7~0) = length - | - std2016 - |
    First = 0x10,
    Consecutive = 0x20,
    FlowControl = 0x30,
}

impl Into<u8> for FrameType {
    #[inline]
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for FrameType {
    type Error = Iso15765Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0xF0 {
            0x00 => Ok(Self::Single),
            0x10 => Ok(Self::First),
            0x20 => Ok(Self::Consecutive),
            0x30 => Ok(Self::FlowControl),
            v => Err(Iso15765Error::InvalidParam(format!("`frame type`({})", v))),
        }
    }
}

/// Flow control type define.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowControlState {
    #[default]
    Continues = 0x00,
    Wait = 0x01,
    Overload = 0x02,
}

impl TryFrom<u8> for FlowControlState {
    type Error = Iso15765Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Continues),
            0x01 => Ok(Self::Wait),
            0x02 => Ok(Self::Overload),
            v => Err(Iso15765Error::InvalidParam(format!("`state` ({})", v))),
        }
    }
}

impl Into<u8> for FlowControlState {
    #[inline]
    fn into(self) -> u8 {
        self as u8
    }
}

/// Flow control frame context.
#[derive(Debug, Default, Copy, Clone)]
pub struct FlowControlContext {
    state: FlowControlState,
    block_size: u8,
    /// Use milliseconds (ms) for values in the range 00 to 7F (0 ms to 127 ms).
    /// If st_min is 0, set to default value. See [`ST_MIN_ISO15765_2`]
    /// and [`ST_MIN_ISO15765_4`]
    ///
    /// Use microseconds (μs) for values in the range F1 to F9 (100 μs to 900 μs).
    ///
    /// Values in the ranges 80 to F0 and FA to FF are reserved.
    st_min: u8,
}

impl FlowControlContext {
    #[inline]
    pub fn new(
        state: FlowControlState,
        block_size: u8,
        st_min: u8,
    ) -> Result<Self, Iso15765Error> {
        match st_min {
            0x80..=0xF0 |
            0xFA..=0xFF => Err(Iso15765Error::InvalidStMin(st_min)),
            v => Ok(Self { state, block_size, st_min: v }),
        }
    }
    #[inline]
    pub fn state(&self) -> FlowControlState {
        self.state
    }
    #[inline]
    pub fn block_size(&self) -> u8 {
        self.block_size
    }
    #[inline]
    pub fn st_min(&self) -> u8 {
        self.st_min
    }
    #[inline]
    pub fn st_min_us(&self) -> u32 {
        match self.st_min {
            // 0x00 => 1000 * 10,
            ..=0x7F => 1000 * (self.st_min as u32),
            0x80..=0xF0 |
            0xFA..=0xFF => {
                // should not enter
                let message = format!("ISO 15765-2 - got an invalid st_min: {}", self.st_min);
                log::error!("{}" ,message);
                unreachable!("{}", message)   // panic is dangerous
            },
            0xF1..=0xF9 => 100 * (self.st_min & 0x0F) as u32,
        }
    }
}

/// byte order define.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ByteOrder {
    /// Motorola byte order
    Big,
    /// Intel byte order
    #[default]
    Little,
    /// The native byte order depends on your CPU
    Native,
}

// #[derive(Debug, Clone)]
// pub enum IsoTpFrame {
//     /// The ISO 15765-2 single frame.
//     SingleFrame { data: Vec<u8> },
//     /// The ISO 15765-2 first frame.
//     FirstFrame { length: u32, data: Vec<u8> },
//     /// The ISO 15765-2 consecutive frame.
//     ConsecutiveFrame { sequence: u8, data: Vec<u8> },
//     /// The ISO 15765-2 flow control frame.
//     FlowControlFrame(FlowControlContext)
// }
//
// impl IsoTpFrame {
//     #[inline]
//     pub fn flow_ctrl_frame(
//         state: FlowControlState,
//         block_size: u8,
//         st_min: u8,
//     ) -> Result<Self, IsoTpError> {
//         Ok(Self::FlowControlFrame(FlowControlContext::new(state, block_size, st_min)?))
//     }
//
//     #[inline]
//     pub fn default_flow_ctrl_frame() -> Self {
//         Self::flow_ctrl_frame(
//             FlowControlState::Continues,
//             ISO_TP_DEFAULT_BLOCK_SIZE,
//             ISO_TP_DEFAULT_ST_MIN
//         )
//             .unwrap()
//     }
// }
