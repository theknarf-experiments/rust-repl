use std::process::Command;

fn main() {
		println!("cargo:rerun-if-changed=Cargo.toml");
		patch_crate::run().expect("Failed while patching");

		// Run `rustc --print sysroot` to get the toolchain’s sysroot path
		let sysroot = Command::new("rustc")
				.arg("--print")
				.arg("sysroot")
				.output()
				.expect("Failed to run rustc --print sysroot");

		let sysroot_str = String::from_utf8(sysroot.stdout)
				.expect("Non-UTF8 output from rustc --print sysroot")
				.trim()
				.to_owned();

		// Emit a linker argument to set the RPATH to "<sysroot>/lib"
		// `cargo:rustc-link-arg` allows us to pass arbitrary flags to the final linker invocation.
		println!("cargo:rustc-link-arg=-Wl,-rpath,{}", format!("{}/lib", sysroot_str));
}
