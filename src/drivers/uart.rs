// uart.rs
// Stephen Marz
// 2-Aug-2020

use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
const UART_BASE: usize = 0x1001_0000;

// #[repr(usize)]
// pub enum UartRegs {
//     TxData = 0,
//     RxData,
//     TxCtrl,
//     RxCtrl,
//     Ie,
//     Ip,
//     Div
// }

// fn reg(r: UartRegs) -> usize {
//     r as usize
// }

pub struct Uart;
struct UartRegs {
    tx_data: u32,
    rx_data: u32,
    tx_ctrl: u32,
    rx_ctrl: u32,
    ie: u32,
    ip: u32,
    div: u32,
}
impl Uart {
    pub fn init() {
        unsafe {
            let u = &mut *(UART_BASE as *mut UartRegs);
            write_volatile(&mut u.div, 4340);
            write_volatile(&mut u.tx_ctrl, 1);
            write_volatile(&mut u.rx_ctrl, 1);
            write_volatile(&mut u.ie, 0);
        }
        
    }
    pub fn put(c: u8) {
        unsafe {
            let u = &mut *(UART_BASE as *mut UartRegs);
            while read_volatile(&mut u.tx_data) >> 31 == 1 {

            }
            write_volatile(&mut u.tx_data, c as u32);
        }
    }
    pub fn get() -> u8 {
        unsafe {
            let u = &mut *(UART_BASE as *mut UartRegs);
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







