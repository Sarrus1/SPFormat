fn main() {
    let src_path = std::path::PathBuf::from("vendor/llvm-project/llvm");
    let clang_include = src_path.join("include/");
    let llvm_include = src_path.join("../llvm/include/");
    let llvm_include_overlay = src_path.join("../utils/bazel/llvm-project-overlay/llvm/include/");
    // vendor\llvm-project\utils\bazel\llvm-project-overlay\llvm\include
    // let mut b = autocxx_build::Builder::new(
    //     "src/main.rs",
    //     &[
    //         &src_path,
    //         &clang_include,
    //         &llvm_include,
    //         &llvm_include_overlay,
    //     ],
    // )
    // .extra_clang_args(&["-std=c++17"])
    // .build()
    // .unwrap();

    // b.flag_if_supported("-std=c++17") // use "-std:c++17" here if using msvc on windows
    //     .compiler("clang++")
    //     .compile("autocxx-demo"); // arbitrary library name, pick anything
    // println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    let dst =
        cmake::Config::new("/Users/charleszablit/Public/foo/SPFormat/vendor/llvm-project/llvm")
            // .configure_arg("-S /Users/charleszablit/Public/foo/SPFormat/vendor/llvm-project/llvm")
            // .configure_arg("-B /Users/charleszablit/Public/foo/SPFormat/vendor/llvm-project/build")
            .configure_arg("-DBUILD_SHARED_LIBS=OFF")
            .configure_arg("-DLLVM_ENABLE_PROJECTS=clang;clang-tools-extra")
            .build_target("clang-format")
            .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
