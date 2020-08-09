// uart.rs
// Stephen Marz
// 2-Aug-2020

use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
use crate::platform::UARTHS_BASE;
const UART_DIV: u32 = 3385;

pub struct Uart;
struct UartRegs {
    tx_data: u32, //0
    rx_data: u32, //4
    tx_ctrl: u32, //8
    rx_ctrl: u32, //c
    ie: u32, //10
    ip: u32, //14
    div: u32, //18
}
impl Uart {
    pub fn init() {
        unsafe {
            let u = &mut *(UARTHS_BASE as *mut UartRegs);
            write_volatile(&mut u.div, UART_DIV);
            write_volatile(&mut u.tx_ctrl, 1);
            write_volatile(&mut u.rx_ctrl, 1);
            write_volatile(&mut u.ie, 0);
        }
        
    }
    pub fn put(c: u8) {
        unsafe {
            let u = &mut *(UARTHS_BASE as *mut UartRegs);
            while read_volatile(&mut u.tx_data) >> 31 == 1 {

            }
            write_volatile(&mut u.tx_data, c as u32);
        }
    }
    pub fn get() -> u8 {
        unsafe {
            let u = &mut *(UARTHS_BASE as *mut UartRegs);
            let r = read_volatile(&mut u.rx_data);
            if r >> 31 == 1 {
                0
            }
            else {
                r as u8
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for i in s.bytes() {
            Self::put(i);
        }
        Ok(())
    }
}







