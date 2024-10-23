#![feature(allocator_api)]
#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate core;
extern crate alloc;

mod rt;

use libc::c_int;
use libc::c_char;

use libc::EXIT_SUCCESS;

use libc::printf;

#[no_mangle]
pub unsafe extern "C" fn main(
	argc: c_int,
	argv: *const *const c_char)
		-> c_int
{
	match argc {
		1 => { printf(c"I am %s and received no optional arguments from the system.\n".as_ptr(), *argv); }
		2 => { printf(c"I am %s and received the following optional argument from the system: %s\n".as_ptr(), *argv, *argv.offset(1)); }
		_ => {
			printf(c"I am %s and received the following optional arguments from the system:".as_ptr(), *argv);
			for i in 1..argc {
				printf(c" %s".as_ptr(), *argv.offset(i as isize));
			}
			printf(c"\n".as_ptr());
		}
	}

	EXIT_SUCCESS
}
