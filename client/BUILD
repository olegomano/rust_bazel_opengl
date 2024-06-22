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
      "//cargo:gl_generator",
      "//cargo:cfg_aliases"
    ]
)

rust_binary(
  name = "main",
  srcs = ["src/final_main.rs"],
  deps = [
      "//client/src/common:ecs",
      "//client/src/render/geometry:geometry",
      "//client/src/render/geometry:material",
      "//client/src/render:gl_context",
      "//client/src/render/geometry:mesh",
      "//cargo:winit",
      "//cargo:glutin",
      "//cargo:glutin_winit",
      "//cargo:glam",
      "//assets:asset_rs",
      "//client/src/common:app_context",
      "//client/src/common:key_manager",
  ],
  proc_macro_deps = [
    "//client/src/common:ecs_proc_macro",
  ]
)
