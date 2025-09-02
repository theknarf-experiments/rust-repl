# rust-repl

A repl for Rust.

## Running

After a fresh clone, run the following:

```bash
git clone <repository-url>
cd rust-repl
# install the patch tool once (if not already present)
cargo install patch-crate
# apply patches to dependencies
cargo patch-crate
# build and start the REPL
cargo run
```

Inside the REPL you can execute Rust code, for example:

```
>> println!("test");
```
