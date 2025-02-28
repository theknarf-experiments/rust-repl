#[allow(static_mut_refs)]
/*
#![feature(link_llvm_intrinsics)]
#[allow(internal_features)]
extern crate libc; // 0.2.42

use std::mem;
use std::time::{SystemTime};

const WIDTH: usize = mem::size_of::<usize>()*8;
type Checkpoint = [i8; WIDTH];

static mut JUMP_POINT: Checkpoint = [0; WIDTH];

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(a: *mut i8) -> i32;
    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(a: *mut i8, b: i32) -> ();
}

fn now() {
    let now = SystemTime::now();
    println!("Now: {:?}", now);

    let ptr: *mut Checkpoint = unsafe { &raw mut JUMP_POINT };
    let franken_pointer: *mut i8 = unsafe { mem::transmute(ptr) };
    unsafe { longjmp(franken_pointer, 1) };
}

fn main() {
    //let mut buff: Checkpoint = [0; WIDTH];
    let ptr: *mut Checkpoint = unsafe { &raw mut JUMP_POINT } ;
    let franken_pointer: *mut i8 = unsafe { mem::transmute(ptr) };

    let rc = unsafe { setjmp(franken_pointer) };
    print!("{}\n", rc);
    if rc != 0 {
        println!("early return!");
    } else {
        println!("jump point was successfully set.");
        now();
    }
}
*/

#[repr(C)]
pub struct JmpBuf {
    // This is *not* guaranteed to match your platform’s jmp_buf layout,
    // so you need to be absolutely sure about the exact size/alignment!
    // For x86_64 glibc, jmp_buf is 8 * 16 bytes = 128 bytes (plus alignment).
    // But this *can* vary across arch, OS, compiler, etc.
    pub buf: [u64; 16],
}

// from libc 0.2.XX or so:
extern "C" {
    fn setjmp(env: *mut JmpBuf) -> ::libc::c_int;
    fn longjmp(env: *mut JmpBuf, val: ::libc::c_int) -> !;
}

static mut GLOBAL_JUMP_BUFFER: JmpBuf = JmpBuf {
    buf: [0; 16],
};

fn main() {
    unsafe {
        let rc = setjmp(&raw mut GLOBAL_JUMP_BUFFER as *mut _);
        println!("setjmp returned {rc}");
        if rc == 0 {
            println!("Calling longjmp...");
            longjmp(&raw mut GLOBAL_JUMP_BUFFER as *mut _, 1234);
        } else {
            println!("Returned via longjmp: rc = {rc}");
        }
    }
}

