// uart.rs
// Stephen Marz
// 2-Aug-2020

use core::fmt::Write;

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

pub struct Uart {
    tx_data: u32,
    rx_data: u32,
    tx_ctrl: u32,
    rx_ctrl: u32,
    ie: u32,
    ip: u32,
    div: u32,
}
impl Uart {
    pub fn new() -> Self {
        Self {
            tx_data: 0,
            rx_data: 0,
            tx_ctrl: 0,
            rx_ctrl: 0,
            ie: 0,
            ip: 0,
            div: 0,
        }
    }
    pub fn init() {
        let u = UART_BASE as *mut Uart;
        unsafe {
            (*u).ie = 0;
            (*u).div = 4340;
            (*u).tx_ctrl = 1;
            (*u).rx_ctrl = 1;
        }
        // let ub = UART_BASE as *mut u32;
        // unsafe {
        //     ub.add(reg(UartRegs::Ie)).write_volatile(0);
        //     ub.add(reg(UartRegs::Div)).write_volatile(4340);
        //     ub.add(reg(UartRegs::TxCtrl)).write_volatile(1);
        //     ub.add(reg(UartRegs::RxCtrl)).write_volatile(1);
        // }
    }
    pub fn put(c: char) {
        let u = UART_BASE as *mut Uart;
        unsafe {
            loop {
                let r = (*u).tx_data >> 31 == 1;
                if !r {
                    (*u).tx_data = c as u32;
                    break;
                }
            }
        }
        // let ub = UART_BASE as *mut u32;
        // unsafe {
        //     while ub.add(reg(UartRegs::TxData)).read_volatile() >> 31 == 1 {
        //     }
        //     ub.add(reg(UartRegs::TxData)).write_volatile(c as u32);
        // }
    }
    pub fn get() -> char {
        // let ub = UART_BASE as *mut u32;
        let u = UART_BASE as *mut Uart;
        unsafe {
            // let ret = ub.add(reg(UartRegs::RxData)).read_volatile();
            let ret = (*u).rx_data;
            if ret >> 31 == 1 {
                0 as char
            }
            else {
                ret as u8 as char
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for i in s.bytes() {
            Self::put(i as char);
        }
        Ok(())
    }
}







