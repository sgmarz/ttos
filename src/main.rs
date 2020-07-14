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

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			// let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
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

pub fn get_hartid() -> usize {
	let ret: usize;
	unsafe {
		asm!("csrr {x}, mhartid", x = out(reg) ret);
	}
	ret
}

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C" fn kinit() {
	let hart = get_hartid();
	// rust_switch_to_user(sched::schedule());
	// switch_to_user will not return, so we should never get here
	if hart == 0 {
		// Bootstrap hart
	}
	
}

pub mod kmem;



