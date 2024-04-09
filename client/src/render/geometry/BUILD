load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_proc_macro")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

rust_library(
  name = "mesh",
  srcs = ["mesh.rs"],
  deps = [
    "//client/src/render:gl_context",
    "//client/src/render:gl_error",
    "//cargo:glam",
    "//client/src/render:texture",
    "//client/src/render:vbo",
    "//client/src/render:shader",
    ":material",
  ],
  proc_macro_deps = [
    "//client/src/render:gl_utils"
  ]
)

rust_library(
  name = "material",
  srcs = ["material.rs"],
  deps = [
    "//client/src/render:texture"
  ],
)

rust_library(
  name = "geometry",
  srcs = ["geometry.rs"],
  deps = [
    ":mesh",
    ":material",
  ],
)
