fn main () {
    println!(r"cargo:rustc-link-search=../Wingine/wingine_c/build");
    println!(r"cargo:rustc-link-search=../Wingine/build");
    println!(r"cargo:rustc-link-search=../HConLib/lib");
    println!(r"cargo:rustc-link-search=../flawed/lib");

    println!(r"cargo:rustc-link-lib=Wingine");
    println!(r"cargo:rustc-link-lib=Winval");
    println!(r"cargo:rustc-link-lib=X11");
    println!(r"cargo:rustc-link-lib=flawed");
    println!(r"cargo:rustc-link-lib=vulkan");
    println!(r"cargo:rustc-link-lib=stdc++");
}
