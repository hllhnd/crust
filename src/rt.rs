mod alloc;
mod panic;

use libc::FILE;

#[link(name = "c")] // cargo doesn't want to link against libc by default in no_std binaries so it needs to be told to do that
extern "C"
{
	// C spec says that `stdout` is a macro but glibc (and probably most other libcs, hopefully) just make it a file handle,
	// but the libc crate still doesn't export it because it's coupling to glibc's implementation.
	pub static stderr: *mut FILE;
}
