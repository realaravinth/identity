mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, aaa!");
}

use pow_sha256::PoW;
use serde::Serialize;

#[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct Pow(PoW<Vec<u8>>);

#[wasm_bindgen]
pub fn gen_pow(difficulty: String, secret: String) -> String {
    let difficulty_int: u128 = difficulty.parse().unwrap();
    let a = PoW::prove_work(&secret.as_bytes().to_vec(), difficulty_int).unwrap();
    println!("nonce: {}", &a.nonce);
    println!("result: {}", &a.result);

    let payload = serde_json::to_string(&Pow(a)).unwrap();
    println!("{:#?}", &payload);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let payload = gen_pow(
            "340282332892701771369528261094307468279".into(),
            "MFsqLMZId629Dh2hrtux2Qdn3gBzCaSt".into(),
        );
        assert_eq!("{\"nonce\":4232799,\"result\":\"340282346372480443202315936306086885027\",\"_spook\":null}", &payload);
    }
}
