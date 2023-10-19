use extism::Plugin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyStruct {
    value: usize,
}

impl MyStruct {
    pub fn get_value(&self) -> usize {
        self.value
    }
}

fn main() {
    let wasm = include_bytes!("../target/wasm32-unknown-unknown/release/plugin.wasm");

    let mut plugin = Plugin::create(wasm, [], false).unwrap();
    let data = plugin.call("start", "");

    println!("{:?}", data);
}
