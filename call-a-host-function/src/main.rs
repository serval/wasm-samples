#[link(wasm_import_module = "serval")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 31;
    let b = 11;
    let c = unsafe { add(a, b) };

    println!("Hello, world! Let's do some math via a WASM import: {a} + {b} = {c}");
}
