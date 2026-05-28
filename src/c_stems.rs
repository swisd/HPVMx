use core::arch::asm;

#[macro_export]
macro_rules! HALT_AND_DEBUG {
    () => {
        unsafe {
            asm!("int3");
            asm!("ud2");
        }
    };
}


// sizeof=0x10
#[repr(C)]
pub struct _GUID {
    Data1: u32,       // 4 byte
    Data2: u16,       // 2 byte
    Data3: u16,       // 2 byte
    Data4: [u8; 8],   // 8 byte
}



pub static mut __SECURITY_COOKIE: usize = 0x4850564D78;

pub unsafe fn __security_check_cookie(StackCookie: usize) {
    if StackCookie != __SECURITY_COOKIE {
        HALT_AND_DEBUG!();
    }
}

pub type HALF_PTR = i16;


pub type HANDLE = *mut core::ffi::c_void;
pub type HANDLE64 = *mut core::ffi::c_void;
pub type HANDLE_PTR = u32;


pub type ULONG_PTR = u32;
pub type SIZE_T = ULONG_PTR;
pub type LPVOID = *mut u8;
pub type _DWORD = u32;
pub type _WORD = u16;

#[repr(u32)]
pub enum HANDLE_FLAGS {
    HANDLE_FLAG_UNK = 0x0,
}

pub unsafe fn ThreadSafeAddRef(pointer: *mut i32) -> i32 {
    let value: i32; // eax

    value = core::sync::atomic::AtomicI32::fetch_add(&mut *(pointer.offset(1) as *mut core::sync::atomic::AtomicI32), 1, core::sync::atomic::Ordering::SeqCst);
    if value <= 0 {
        breakpoint();
        BUG();
    }
    if value == 0x7FFFFFFF {
        breakpoint();
        BUG();
    }
    1
}


pub fn breakpoint(){}
pub fn BUG(){}



pub unsafe fn validate_and_execute(p_callback: i32, p_data: *const u32, p_out_status: *mut u32) -> i32 {
    *p_out_status = 0;

    let data = core::slice::from_raw_parts(p_data, 4);

    const SIG_A: [u32; 4] = [0, 0, 0x000000C0, 0x46000000];
    const SIG_B: [u32; 4] = [0x4850564D, 0x582D5349, 0x474E2D76, 0x322E3078];


    if data == SIG_A || data == SIG_B {
        *p_out_status = p_callback as u32;


        let vtable = *(p_callback as *const *const fn());
        let callback_fn = *vtable.add(1);

        callback_fn();
        return 0;
    }

    -255
}




/*
static unsigned char heap[1024 * 1024]
static unsigned char* heap_ptr = heap;

void* bump_alloc(size_t size)
{
    void* p = heap_ptr;
    heap_ptr += size;
    return p;
}

*/

static mut SIMPLE_HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];
static mut SIMPLE_HEAP_PTR: usize = 0;

fn bump_alloc(size: usize) -> *mut u8 {
    unsafe {
        let p = SIMPLE_HEAP.as_mut_ptr().add(SIMPLE_HEAP_PTR);
        SIMPLE_HEAP_PTR += size;
        p
    }
}

/*
int next_16(int n)
{
    return ((n + 15) >> 4) << 4;
}
 */
pub fn next_16(n: i32) -> i32 {
    ((n + 15) >> 4) << 4
}



// -------------------------------------------------------------
/*
static unsigned char heap[1024 * 1024]
static unsigned char* heap_ptr = heap;

void* bump_alloc(size_t size)
{
    if (heap_ptr + size > heap + sizeof(heap)) {
        return NULL;
    }
    void* p = heap_ptr;

    heap_ptr += size;
    return p;
}

int next_16(int n)
{
    return ((n + 15) >> 4) << 4;
}

void free(void* ptr)
{
}

struct block
{
    size_t size;
    int free;
    struct block* next;
}

*/

/*
#include <stddef.h>

// 1MB Heap space as you defined
static unsigned char heap[1024 * 1024];

struct block
{
    size_t size;
    int free;
    struct block* next;
};

// Start with a pointer to the very beginning of our heap
static struct block* free_list = (struct block*)heap;
static int initialized = 0;

// Aligning to 16 bytes for performance and architecture requirements
size_t align_16(size_t size)
{
    return ((size + 15) >> 4) << 4;
}

// Initialize the heap on the first allocation call
void init_heap()
{
    free_list->size = sizeof(heap) - sizeof(struct block);
    free_list->free = 1;
    free_list->next = NULL;
    initialized = 1;
}

void* my_alloc(size_t size)
{
    if (!initialized) {
        init_heap();
    }

    // Align the requested size
    size = align_16(size);

    struct block* curr = free_list;

    // Traverse the linked list to find a suitable free block
    while (curr != NULL) {
        if (curr->free && curr->size >= size) {

            // Optional: Split the block if it's significantly larger than requested
            // We need enough space for the requested size, a new block header, and some data
            if (curr->size >= size + sizeof(struct block) + 16) {
                struct block* new_block = (struct block*)((unsigned char*)curr + sizeof(struct block) + size);

                new_block->size = curr->size - size - sizeof(struct block);
                new_block->free = 1;
                new_block->next = curr->next;

                curr->size = size;
                curr->next = new_block;
            }

            curr->free = 0;

            // Return a pointer to the memory *after* the block header
            return (void*)(curr + 1);
        }
        curr = curr->next;
    }

    return NULL; // Out of memory / no suitable block found
}

void my_free(void* ptr)
{
    if (!ptr) return;

    // Get the pointer to the block header by stepping backward
    struct block* block_ptr = (struct block*)ptr - 1;
    block_ptr->free = 1;

    // Optional optimization: Coalescing (combining adjacent free blocks)
    struct pointer = free_list;
    while (pointer != NULL) {
        if (pointer->free && pointer->next && pointer->next->free) {
            pointer->size += sizeof(struct block) + pointer->next->size;
            pointer->next = pointer->next->next;
        }
        pointer = pointer->next;
    }
}
*/
const HEAP_SIZE: usize = 1024 * 1024;

#[repr(C)]
struct Block {
    size: usize,
    free: bool,
    next: *mut Block,
}


struct Heap {
    heap: core::cell::UnsafeCell<[u8; HEAP_SIZE]>,
    free_list: *mut Block,
    initialized: bool,
}

impl Heap {
    const fn new() -> Self {
        Self {
            heap: core::cell::UnsafeCell::new([0; HEAP_SIZE]),
            free_list: core::ptr::null_mut(),
            initialized: false,
        }
    }

    fn align_16(size: usize) -> usize {
        (size + 15) & !15
    }

    unsafe fn init_heap(&mut self) {
        let heap_ptr = self.heap.get() as *mut u8;
        self.free_list = heap_ptr as *mut Block;
        (*self.free_list).size = HEAP_SIZE - size_of::<Block>();
        (*self.free_list).free = true;
        (*self.free_list).next = core::ptr::null_mut();
        self.initialized = true;
    }

    unsafe fn my_alloc(&mut self, size: usize) -> *mut u8 {
        if !self.initialized {
            self.init_heap();
        }

        let size = Self::align_16(size);
        let mut curr = self.free_list;

        while !curr.is_null() {
            if (*curr).free && (*curr).size >= size {
                // Split block if large enough
                if (*curr).size >= size + size_of::<Block>() + 16 {
                    let new_block_ptr = (curr as *mut u8).add(size_of::<Block>() + size) as *mut Block;

                    (*new_block_ptr).size = (*curr).size - size - size_of::<Block>();
                    (*new_block_ptr).free = true;
                    (*new_block_ptr).next = (*curr).next;

                    (*curr).size = size;
                    (*curr).next = new_block_ptr;
                }

                (*curr).free = false;
                return (curr as *mut u8).add(size_of::<Block>());
            }
            curr = (*curr).next;
        }

        core::ptr::null_mut()
    }

    unsafe fn my_free(&mut self, ptr: *mut u8) {
        if ptr.is_null() {
            return;
        }

        let block_ptr = (ptr as *mut Block).offset(-1);
        (*block_ptr).free = true;

        // Coalescing adjacent free blocks
        let mut pointer = self.free_list;
        while !pointer.is_null() {
            if (*pointer).free {
                let next = (*pointer).next;
                if !next.is_null() && (*next).free {
                    (*pointer).size += size_of::<Block>() + (*next).size;
                    (*pointer).next = (*next).next;
                    continue; // Check again with new next
                }
            }
            pointer = (*pointer).next;
        }
    }
}

// Global heap instance
static mut HEAP: Heap = Heap::new();

pub unsafe fn my_alloc(size: usize) -> *mut u8 {
    HEAP.my_alloc(size)
}

pub unsafe fn my_free(ptr: *mut u8) {
    HEAP.my_free(ptr)
}