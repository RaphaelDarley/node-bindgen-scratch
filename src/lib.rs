use node_bindgen::derive::node_bindgen;

pub struct Test {
    foo: f64,
}

#[node_bindgen]
impl Test {
    #[node_bindgen(constructor)]
    fn init(foo: f64) -> Self {
        Self { foo }
    }

    #[node_bindgen(getter)]
    fn foo(&self) -> bool {
        self.foo == 10.0
    }
}
