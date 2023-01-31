use serval::invoke_extension;

#[link(wasm_import_module = "as-fib")]
extern "C" {
    fn fib(n: i32) -> i32;
}

fn main() {
    println!("Calling a WASM extension...");
    for i in 1..10 {
        println!("Fibonacci #{i} = {}", unsafe { fib(i) });
    }

    println!("");

    println!("Invoking the birdfeeder capability...");
    match invoke_extension("birdfeeder".to_string(), &"forks".as_bytes().to_vec()) {
        Ok(buf) => {
            let resp = String::from_utf8_lossy(&buf);
            println!("Got response from host: {resp}");
        }
        Err(err) => panic!("Invocation failed: {err:?}"),
    };
}
