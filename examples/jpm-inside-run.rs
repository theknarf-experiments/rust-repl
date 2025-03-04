#![feature(rustc_private)]
use rust_repl::run;

fn main() {
    let src_code = format!(r#"
#![feature(rustc_private)]
#[allow(static_mut_refs)]

extern crate libc;

#[repr(C)]
pub struct JmpBuf {{
    // This is *not* guaranteed to match your platform’s jmp_buf layout,
    // so you need to be absolutely sure about the exact size/alignment!
    // For x86_64 glibc, jmp_buf is 8 * 16 bytes = 128 bytes (plus alignment).
    // But this *can* vary across arch, OS, compiler, etc.
    pub buf: [u64; 16],
}}

// from libc 0.2.XX or so:
extern "C" {{
    fn setjmp(env: *mut JmpBuf) -> ::libc::c_int;
    fn longjmp(env: *mut JmpBuf, val: ::libc::c_int) -> !;
}}

static mut GLOBAL_JUMP_BUFFER: JmpBuf = JmpBuf {{
    buf: [0; 16],
}};

fn main() {{
    unsafe {{
        let rc = setjmp(&raw mut GLOBAL_JUMP_BUFFER as *mut _);
        println!("setjmp returned {{rc}}");
        if rc == 0 {{
            println!("Calling longjmp...");
            longjmp(&raw mut GLOBAL_JUMP_BUFFER as *mut _, 1234);
        }} else {{
            println!("Returned via longjmp: rc = {{rc}}");
        }}
    }}
}}
    "#);

    run(src_code);
}
