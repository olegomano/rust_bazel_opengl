load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

# collect the outputs.
cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    # Optional environment variables passed during build.rs compilation
    rustc_env = {
       "CARGO_PKG_VERSION": "0.1.2",
    },
    deps = [
      "//cargo:gl_generator",
      "//cargo:cfg_aliases"
    ]
)

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
      "//cargo:winit",
      "//cargo:glutin",
      "//cargo:glutin_winit",
      "//cargo:raw_window_handle",
      ":build_script"
    ]
)
