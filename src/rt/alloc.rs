use core::alloc::GlobalAlloc;
use core::alloc::Layout;

use libc::c_void;

use libc::calloc;
use libc::free;
use libc::malloc;
use libc::realloc;

#[global_allocator]
static ALLOCATOR: LibcAllocator = LibcAllocator;

struct LibcAllocator;

unsafe impl GlobalAlloc for LibcAllocator
{
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout)
			-> *mut u8
	{
		malloc(layout.size()) as *mut u8
	}

	#[inline(always)]
	unsafe fn dealloc(
		&self,
		ptr:     *mut u8,
		_layout: Layout)
	{
		free(ptr as *mut c_void)
	}

	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout)
			-> *mut u8
	{
		calloc(1, layout.size()) as *mut u8
	}

	#[inline(always)]
	unsafe fn realloc(
		&self,
		ptr:      *mut u8,
		_layout:  Layout,
		new_size: usize)
			-> *mut u8
	{
		realloc(ptr as *mut c_void, new_size) as *mut u8
	}
}
