use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn document_ready() {
    coast::ready().await;
}
