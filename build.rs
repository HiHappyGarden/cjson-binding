fn main() {
    // Only link to cJSON libraries when building tests (with std feature)
    #[cfg(feature = "std")]
    {
        // Add search path for cJSON library
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
        
        // Link to cJSON library
        println!("cargo:rustc-link-lib=cjson");
        println!("cargo:rustc-link-lib=cjson_utils");
    }
}
