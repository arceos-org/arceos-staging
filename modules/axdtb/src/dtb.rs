extern crate alloc;

use alloc::{borrow::ToOwned, vec::Vec};
use hermit_dtb::Dtb;

pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub enum DtbHelperError {
    BadFileFormat,
}

pub struct DtbHelper<'a> {
    dtb: Option<Dtb<'a>>,
}

impl<'a> DtbHelper<'a> {
    pub fn new(dtb_pa: usize) -> Self {
        Self {
            dtb: unsafe { Dtb::from_raw(dtb_pa as *const u8) },
        }
    }

    pub fn parse(&self) -> Result<DtbInfo, DtbHelperError> {
        match self.parse_memory() {
            Some((memory_addr, memory_size)) => {
                let mmio_regions = self.parse_virtio_mmio();
                Ok(DtbInfo {
                    memory_addr,
                    memory_size,
                    mmio_regions,
                })
            }
            None => {
                Err(DtbHelperError::BadFileFormat)
            }
        }
    }

    fn parse_memory(&self) -> Option<(usize, usize)> {
        self.dtb.as_ref().unwrap()
            .enum_subnodes("/")
            .filter(|&name| name.starts_with("memory"))
            .map(|name| self.parse_dtb_property(&name))
            .nth(0)
    }

    fn parse_virtio_mmio(&self) -> Vec<(usize, usize)> {
        let root_path = "/soc";
        self.dtb.as_ref().unwrap()
            .enum_subnodes(root_path)
            .filter(|&name| name.starts_with("virtio_mmio"))
            .map(|name| self.parse_dtb_property(&[root_path, name].join("/")))
            .collect()
    }

    fn parse_dtb_property(&self, path: &str) -> (usize, usize) {
        match self.dtb.as_ref().unwrap().get_property(&path, "reg") {
            Some(reg) => self.parse_reg_bytes(reg),
            _ => (0, 0),
        }
    }

    fn parse_reg_bytes(&self, reg: &[u8]) -> (usize, usize) {
        (
            self.bytes_to_usize(&reg[..8]),
            self.bytes_to_usize(&reg[8..]),
        )
    }

    fn bytes_to_usize(&self, bytes: &[u8]) -> usize {
        usize::from_be_bytes(bytes.to_owned().try_into().unwrap())
    }
}