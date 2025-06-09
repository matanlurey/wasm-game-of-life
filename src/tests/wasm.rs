use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn pass() {
    let a = 1;
    let b = 2;
    assert_eq!(a, b - 1);
}

#[cfg(test)]
mod not_wasm_tests {
    #[test]
    fn test_not_wasm() {
        let a = 1;
        let b = 2;
        assert_eq!(a, b - 1);
    }
}
