fn main() {
    #[cfg(feature = "uni_ffi")]
    {
        uniffi::generate_scaffolding("src/indy2_vdr.udl").unwrap();
    }
}
