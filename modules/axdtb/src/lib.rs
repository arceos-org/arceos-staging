#![no_std]

mod dtb;
pub use dtb::{DtbHelper, DtbInfo, DtbHelperError};

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, DtbHelperError> {
    DtbHelper::new(dtb_pa).parse()
}