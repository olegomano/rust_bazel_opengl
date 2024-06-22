load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

# collect the outputs.
cargo_build_script(
    name = "gl_build_script",
    srcs = ["build.rs"],
    # Optional environment variables passed during build.rs compilation
    rustc_env = {
        "CARGO_PKG_VERSION": "0.1.2",
    },
    deps = [
        "//cargo:cfg_aliases",
        "//cargo:gl_generator",
    ],
)

rust_binary(
    name = "main",
    srcs = ["src/final_main.rs"],
    proc_macro_deps = [
        "//client/src/common:ecs_proc_macro",
    ],
    deps = [
        "//assets:asset_rs",
        "//cargo:glam",
        "//cargo:glutin",
        "//cargo:glutin_winit",
        "//cargo:winit",
        "//client/src/common:app_context",
        "//client/src/common:ecs",
        "//client/src/common:key_manager",
        "//client/src/render:gl_context",
        "//client/src/render/geometry",
        "//client/src/render/geometry:material",
        "//client/src/render/geometry:mesh",
    ],
)
