fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Build PolyHook2 using cmake.
    let build = cmake::Config::new("polyhook")
        .static_crt(true)
        .profile("Release")
        .target("INSTALL")
        .build();

    println!("cargo:rustc-link-search=native={}", build.join("lib").display());
    println!("cargo:rustc-link-lib=static=PolyHook_2");
}