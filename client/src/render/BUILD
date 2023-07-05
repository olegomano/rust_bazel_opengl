load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

rust_library(
  name = "shader",
  srcs = ["shader.rs"],
  deps = [
    "//client:gl_build_script"
  ]
)


rust_library(
  name = "window",
  srcs = ["window.rs"],
  deps = [
      "//client:gl_build_script",
      "//cargo:winit",
      "//cargo:glutin",
      "//cargo:glutin_winit",
      "//cargo:raw_window_handle",
  ]
)
