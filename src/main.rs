// TTOS
// Teeny-tiny OS
// Stephen Marz
// 14 July 2020
#![no_main]
#![no_std]
#![feature(panic_info_message,
           asm,
		   global_asm,
           allocator_api,
           alloc_error_handler,
           alloc_prelude,
		   const_raw_ptr_to_usize_cast,
		   lang_items)]

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("asm/boot.S"));

// #[macro_use]
extern crate alloc;

use core::ptr::{read_volatile, write_volatile};

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::drivers::uart::Uart, $($args)+);
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
		         "line {}, file {}: {}",
		         p.line(),
		         p.file(),
		         info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}
#[no_mangle]
extern "C" fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}

extern "C" {
	fn switch_to_user(frame: usize) -> !;
}

/// Switch to user is an assembly function that loads
/// a frame. Since it will jump to another program counter,
/// it will never return back here. We don't care if we leak
/// the stack, since we will recapture the stack during m_trap.
fn rust_switch_to_user(frame: usize) -> ! {
	unsafe {
		switch_to_user(frame);
	}
}

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C" fn kinit(hart: usize) {
	drivers::uart::Uart::init();
	println!("Hart {}", hart);
	loop {
		let r = drivers::uart::Uart::get();
		if r != 0 {
			let c = r as char;
			match c {
				'\r' => println!(),
				'1' => {
					println!("IPI to hart 1");
					let clint = crate::drivers::clint::Clint::get_mut();
					clint.write_msip(1, 1);
					println!("Time is {}", clint.read_mtime());
				}
				'o' => {
					println!("you pressed it!");
				}
				_ => print!("{}", c),
			}
		}
	}
}

fn ipi() {
	println!("I'm here in the ipi!");
}

fn delay(amt: usize) {
	for _ in 0..amt*2000000 {
		unsafe { asm!("nop"); }
	}
}

pub mod kmem;
pub mod drivers;
pub mod platform;

