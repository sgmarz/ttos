// clint.rs
// Core Local Interruptor Control
// Stephen Marz
// 9-Aug-2020

use core::ptr::{read_volatile, write_volatile};
use crate::platform::CLINT_BASE;

#[repr(C)]
pub struct Clint {
    pub msip: [u32; 4096],
    pub mtimecmp: [u64; 4095],
    pub mtime: u64,
}

impl Clint {
    pub fn get() -> &'static Clint {
        unsafe {
            &*(CLINT_BASE as *const Clint)
        }    
    }
    pub fn get_mut() -> &'static mut Clint {
        unsafe {
            &mut *(CLINT_BASE as *mut Clint)
        }
    }
    pub fn write_msip(&mut self, which: usize, val: u32) {
        unsafe {
            write_volatile(&mut self.msip[which], val);
        }
    }
    pub fn read_msip(&self, which: usize) -> u32 {
        unsafe {
            read_volatile(&self.msip[which])
        }
    }
    pub fn write_mtimecmp(&mut self, which: usize, val: u64) {
        unsafe {
            write_volatile(&mut self.mtimecmp[which], val);
        }
    }
    pub fn read_mtimecmp(&self, which: usize) -> u64 {
        unsafe {
            read_volatile(&self.mtimecmp[which])
        }
    }
    pub fn read_mtime(&self) -> u64 {
        unsafe {
            read_volatile(&self.mtime)
        }
    }
}

