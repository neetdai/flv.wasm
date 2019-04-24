
// Value作为JsValue的过渡, 
// 将Value转为JsValue用JsValue::from_serde(&Value)
// 将JsValue转为Value用JsValue::into_serde()

#[derive(Serialize, Deserialize)]
pub(crate) struct Value<T>(T);

impl<T> Value<T> {
    pub fn new(content: T) -> Self {
        Value(content)
    }
}
