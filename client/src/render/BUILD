load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_proc_macro")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

rust_library(
  name = "gl_context",
  srcs = ["gl_context.rs"],
  deps = [
    "//client:gl_build_script"
  ]
)

rust_proc_macro(
  name = "gl_utils",
  srcs = ["gl_utils.rs"],
  deps = [
    "//client:gl_build_script",
    "//cargo:syn",
    "//cargo:quote"
  ]
)


rust_library(
  name = "shader",
  srcs = ["shader.rs"],
  deps = [
    "//client/src/render:gl_context",
  ],
  proc_macro_deps = [
    ":gl_utils"
  ]
)

rust_library(
  name = "default_shader",
  srcs = ["shaders/default_shader.rs"],
  deps = [
    "//client/src/render:gl_context",
    "//cargo:glam",
    ":shader",
    ":vbo",
  ],
  proc_macro_deps = [
    ":gl_utils"
  ]
)


rust_library(
  name = "drawable",
  srcs = ["drawable/drawable.rs", "drawable/triangle_drawable.rs"],
  deps = [
    "//client/src/render:gl_context",
    "//cargo:glam",
    ":shader",
    ":vbo",
  ],
  proc_macro_deps = [
    ":gl_utils"
  ]
)


rust_library(
  name = "vbo",
  srcs = ["vbo.rs"],
  deps = [
    "//client/src/render:gl_context",
  ]
)


rust_library(
  name = "window",
  srcs = ["window.rs"],
  deps = [
      "//client/src/render:gl_context",
      "//cargo:winit",
      "//cargo:glutin",
      "//cargo:glam",
      "//cargo:glutin_winit",
      "//cargo:raw_window_handle",
  ],
  proc_macro_deps = [
    ":gl_utils"
  ]

)
