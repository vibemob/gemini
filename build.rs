#[allow(unused_imports)]
use std::env;

fn main() {
    // This instruction is only needed when the *target* operating system is macOS or iOS.
    // It will be ignored when compiling for other targets like WebAssembly (wasm).
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        // When building in a Nix shell, the `SDKROOT` variable points to the Apple SDK.
        // We need to explicitly tell the linker where to find the frameworks.
        if let Ok(sdk_root) = env::var("SDKROOT") {
            println!("cargo:rustc-link-search=framework={}/System/Library/Frameworks", sdk_root);
        }
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }
}
