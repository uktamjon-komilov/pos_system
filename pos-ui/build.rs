//! Compiles the Slint UI tree (entry point: ui/app-window.slint) into Rust code.

fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint UI failed to compile");
}
