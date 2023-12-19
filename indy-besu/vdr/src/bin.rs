fn main() {
    #[cfg(feature = "uni_ffi")]
    {
        uniffi::uniffi_bindgen_main()
    }
}
