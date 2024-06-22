load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_proc_macro")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "gl_context",
    srcs = ["gl_context.rs"],
    deps = [
        "//client:gl_build_script",
    ],
)

rust_library(
    name = "gl_error",
    srcs = ["gl_error.rs"],
    deps = [
        "//client/src/render:gl_context",
    ],
)

rust_proc_macro(
    name = "gl_utils",
    srcs = ["gl_utils.rs"],
    deps = [
        "//cargo:quote",
        "//cargo:syn",
        "//client:gl_build_script",
    ],
)

rust_library(
    name = "shader",
    srcs = ["shader.rs"],
    proc_macro_deps = [
        ":gl_utils",
    ],
    deps = [
        "//cargo:glam",
        "//client/src/render:gl_context",
        "//client/src/render:gl_error",
    ],
)

rust_library(
    name = "default_shader",
    srcs = [
        "shaders/default_shader.rs",
    ],
    proc_macro_deps = [
        ":gl_utils",
    ],
    deps = [
        ":shader",
        ":texture",
        ":vbo",
        "//cargo:glam",
        "//client/src/render:gl_context",
    ],
)

rust_library(
    name = "mesh_shader",
    srcs = [
        "shaders/mesh_shader.rs",
    ],
    proc_macro_deps = [
        ":gl_utils",
    ],
    deps = [
        ":shader",
        ":texture",
        ":vbo",
        "//cargo:glam",
        "//client/src/render:gl_context",
    ],
)

rust_library(
    name = "texture",
    srcs = ["texture.rs"],
    proc_macro_deps = [
        ":gl_utils",
    ],
    deps = [
        "//assets:asset",
        "//client/src/render:gl_context",
        "//client/src/render:gl_error",
    ],
)

rust_library(
    name = "vbo",
    srcs = ["vbo.rs"],
    deps = [
        "//client/src/render:gl_context",
        "//client/src/render:gl_error",
    ],
)
