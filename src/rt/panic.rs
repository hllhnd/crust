use core::panic::PanicInfo;

use alloc::ffi::CString;

use libc::EXIT_FAILURE;

use libc::abort;
use libc::exit;
use libc::fprintf;

use crate::rt::stderr;

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> !
{
	let (file, line, column) = if let Some(location) = info.location() {
		(location.file(), location.line(), location.column())
	} else {
		("unknown", 0, 0)
	};

	let message = info.message().as_str().unwrap_or("no message");

	let Ok(file)    = CString::new(file)    else { abort() };
	let Ok(message) = CString::new(message) else { abort() };

	fprintf(
		stderr,
		c"PANIC at %s:%d:%d: %s\n".as_ptr(),
		file.as_ptr(),
		line,
		column,
		message.as_ptr()
	);

	exit(EXIT_FAILURE);
}

// To avoid setting `panic = "abort"` (and foregoing the cool awesome custom panic handler) Rust and/or LLVM
// want many bullshit functions for bullshit things that real programmers don't need, in order to not have a stroke at link time
// in debug builds or whenever LLVM chooses not to optimize out whatever bullshit makes references to these bullshits.
//
// I do not know what any of this does
mod unwind
{
	// It makes sense to just have these trap because it's probably better to find out right away if calls to these are being generated.
	use libc::abort;

	#[lang = "eh_personality"]
	unsafe extern "C" fn eh_personality()
		-> !
	{
		abort()
	}

	#[no_mangle]
	unsafe extern "C" fn _Unwind_Resume()
		-> !
	{
		abort()
	}
}
