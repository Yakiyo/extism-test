use extism_pdk::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MyStruct {
    value: usize,
}

impl MyStruct {
    pub fn get_value(&self) -> usize {
        self.value
    }
}

#[plugin_fn]
pub fn init(_: ()) -> FnResult<Json<MyStruct>> {
    Ok(Json(MyStruct { value: 1 }))
}
