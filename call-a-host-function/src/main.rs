use serval::invoke_capability;

fn main() {
    println!("Invoking the birdfeeder capability...");
    match invoke_capability("birdfeeder".to_string(), &"forks".as_bytes().to_vec()) {
        Ok(buf) => {
            let resp = String::from_utf8_lossy(&buf);
            println!("Got response from host: {resp}");
        }
        Err(err) => panic!("Invocation failed: {err:?}"),
    };
}
