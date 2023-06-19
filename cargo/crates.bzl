"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__ab_glyph__0_2_21",
        url = "https://crates.io/api/v1/crates/ab_glyph/0.2.21/download",
        type = "tar.gz",
        sha256 = "5110f1c78cf582855d895ecd0746b653db010cec6d9f5575293f27934d980a39",
        strip_prefix = "ab_glyph-0.2.21",
        build_file = Label("//cargo/remote:BUILD.ab_glyph-0.2.21.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ab_glyph_rasterizer__0_1_8",
        url = "https://crates.io/api/v1/crates/ab_glyph_rasterizer/0.1.8/download",
        type = "tar.gz",
        sha256 = "c71b1793ee61086797f5c80b6efa2b8ffa6d5dd703f118545808a7f2e27f7046",
        strip_prefix = "ab_glyph_rasterizer-0.1.8",
        build_file = Label("//cargo/remote:BUILD.ab_glyph_rasterizer-0.1.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__adler__1_0_2",
        url = "https://crates.io/api/v1/crates/adler/1.0.2/download",
        type = "tar.gz",
        sha256 = "f26201604c87b1e01bd3d98f8d5d9a8fcbb815e8cedb41ffccbeb4bf593a35fe",
        strip_prefix = "adler-1.0.2",
        build_file = Label("//cargo/remote:BUILD.adler-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__android_activity__0_4_1",
        url = "https://crates.io/api/v1/crates/android-activity/0.4.1/download",
        type = "tar.gz",
        sha256 = "7c77a0045eda8b888c76ea473c2b0515ba6f471d318f8927c5c72240937035a6",
        strip_prefix = "android-activity-0.4.1",
        build_file = Label("//cargo/remote:BUILD.android-activity-0.4.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__android_properties__0_2_2",
        url = "https://crates.io/api/v1/crates/android-properties/0.2.2/download",
        type = "tar.gz",
        sha256 = "fc7eb209b1518d6bb87b283c20095f5228ecda460da70b44f0802523dea6da04",
        strip_prefix = "android-properties-0.2.2",
        build_file = Label("//cargo/remote:BUILD.android-properties-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__arrayref__0_3_7",
        url = "https://crates.io/api/v1/crates/arrayref/0.3.7/download",
        type = "tar.gz",
        sha256 = "6b4930d2cb77ce62f89ee5d5289b4ac049559b1c45539271f5ed4fdc7db34545",
        strip_prefix = "arrayref-0.3.7",
        build_file = Label("//cargo/remote:BUILD.arrayref-0.3.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__arrayvec__0_7_4",
        url = "https://crates.io/api/v1/crates/arrayvec/0.7.4/download",
        type = "tar.gz",
        sha256 = "96d30a06541fbafbc7f82ed10c06164cfbd2c401138f6addd8404629c4b16711",
        strip_prefix = "arrayvec-0.7.4",
        build_file = Label("//cargo/remote:BUILD.arrayvec-0.7.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__autocfg__1_1_0",
        url = "https://crates.io/api/v1/crates/autocfg/1.1.0/download",
        type = "tar.gz",
        sha256 = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa",
        strip_prefix = "autocfg-1.1.0",
        build_file = Label("//cargo/remote:BUILD.autocfg-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bitflags__1_3_2",
        url = "https://crates.io/api/v1/crates/bitflags/1.3.2/download",
        type = "tar.gz",
        sha256 = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a",
        strip_prefix = "bitflags-1.3.2",
        build_file = Label("//cargo/remote:BUILD.bitflags-1.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__block_sys__0_1_0_beta_1",
        url = "https://crates.io/api/v1/crates/block-sys/0.1.0-beta.1/download",
        type = "tar.gz",
        sha256 = "0fa55741ee90902547802152aaf3f8e5248aab7e21468089560d4c8840561146",
        strip_prefix = "block-sys-0.1.0-beta.1",
        build_file = Label("//cargo/remote:BUILD.block-sys-0.1.0-beta.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__block2__0_2_0_alpha_6",
        url = "https://crates.io/api/v1/crates/block2/0.2.0-alpha.6/download",
        type = "tar.gz",
        sha256 = "8dd9e63c1744f755c2f60332b88de39d341e5e86239014ad839bd71c106dec42",
        strip_prefix = "block2-0.2.0-alpha.6",
        build_file = Label("//cargo/remote:BUILD.block2-0.2.0-alpha.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bumpalo__3_13_0",
        url = "https://crates.io/api/v1/crates/bumpalo/3.13.0/download",
        type = "tar.gz",
        sha256 = "a3e2c3daef883ecc1b5d58c15adae93470a91d425f3532ba1695849656af3fc1",
        strip_prefix = "bumpalo-3.13.0",
        build_file = Label("//cargo/remote:BUILD.bumpalo-3.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bytemuck__1_13_1",
        url = "https://crates.io/api/v1/crates/bytemuck/1.13.1/download",
        type = "tar.gz",
        sha256 = "17febce684fd15d89027105661fec94afb475cb995fbc59d2865198446ba2eea",
        strip_prefix = "bytemuck-1.13.1",
        build_file = Label("//cargo/remote:BUILD.bytemuck-1.13.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__calloop__0_10_6",
        url = "https://crates.io/api/v1/crates/calloop/0.10.6/download",
        type = "tar.gz",
        sha256 = "52e0d00eb1ea24371a97d2da6201c6747a633dc6dc1988ef503403b4c59504a8",
        strip_prefix = "calloop-0.10.6",
        build_file = Label("//cargo/remote:BUILD.calloop-0.10.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cc__1_0_79",
        url = "https://crates.io/api/v1/crates/cc/1.0.79/download",
        type = "tar.gz",
        sha256 = "50d30906286121d95be3d479533b458f87493b30a4b5f79a607db8f5d11aa91f",
        strip_prefix = "cc-1.0.79",
        build_file = Label("//cargo/remote:BUILD.cc-1.0.79.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_aliases__0_1_1",
        url = "https://crates.io/api/v1/crates/cfg_aliases/0.1.1/download",
        type = "tar.gz",
        sha256 = "fd16c4719339c4530435d38e511904438d07cce7950afa3718a84ac36c10e89e",
        strip_prefix = "cfg_aliases-0.1.1",
        build_file = Label("//cargo/remote:BUILD.cfg_aliases-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cgl__0_3_2",
        url = "https://crates.io/api/v1/crates/cgl/0.3.2/download",
        type = "tar.gz",
        sha256 = "0ced0551234e87afee12411d535648dd89d2e7f34c78b753395567aff3d447ff",
        strip_prefix = "cgl-0.3.2",
        build_file = Label("//cargo/remote:BUILD.cgl-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation__0_9_3",
        url = "https://crates.io/api/v1/crates/core-foundation/0.9.3/download",
        type = "tar.gz",
        sha256 = "194a7a9e6de53fa55116934067c844d9d749312f75c6f6d0980e8c252f8c2146",
        strip_prefix = "core-foundation-0.9.3",
        build_file = Label("//cargo/remote:BUILD.core-foundation-0.9.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation_sys__0_8_4",
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.8.4/download",
        type = "tar.gz",
        sha256 = "e496a50fda8aacccc86d7529e2c1e0892dbd0f898a6b5645b5561b89c3210efa",
        strip_prefix = "core-foundation-sys-0.8.4",
        build_file = Label("//cargo/remote:BUILD.core-foundation-sys-0.8.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_graphics__0_22_3",
        url = "https://crates.io/api/v1/crates/core-graphics/0.22.3/download",
        type = "tar.gz",
        sha256 = "2581bbab3b8ffc6fcbd550bf46c355135d16e9ff2a6ea032ad6b9bf1d7efe4fb",
        strip_prefix = "core-graphics-0.22.3",
        build_file = Label("//cargo/remote:BUILD.core-graphics-0.22.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_graphics_types__0_1_2",
        url = "https://crates.io/api/v1/crates/core-graphics-types/0.1.2/download",
        type = "tar.gz",
        sha256 = "2bb142d41022986c1d8ff29103a1411c8a3dfad3552f87a4f8dc50d61d4f4e33",
        strip_prefix = "core-graphics-types-0.1.2",
        build_file = Label("//cargo/remote:BUILD.core-graphics-types-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crc32fast__1_3_2",
        url = "https://crates.io/api/v1/crates/crc32fast/1.3.2/download",
        type = "tar.gz",
        sha256 = "b540bd8bc810d3885c6ea91e2018302f68baba2129ab3e88f32389ee9370880d",
        strip_prefix = "crc32fast-1.3.2",
        build_file = Label("//cargo/remote:BUILD.crc32fast-1.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dispatch__0_2_0",
        url = "https://crates.io/api/v1/crates/dispatch/0.2.0/download",
        type = "tar.gz",
        sha256 = "bd0c93bb4b0c6d9b77f4435b0ae98c24d17f1c45b2ff844c6151a07256ca923b",
        strip_prefix = "dispatch-0.2.0",
        build_file = Label("//cargo/remote:BUILD.dispatch-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dlib__0_5_2",
        url = "https://crates.io/api/v1/crates/dlib/0.5.2/download",
        type = "tar.gz",
        sha256 = "330c60081dcc4c72131f8eb70510f1ac07223e5d4163db481a04a0befcffa412",
        strip_prefix = "dlib-0.5.2",
        build_file = Label("//cargo/remote:BUILD.dlib-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__downcast_rs__1_2_0",
        url = "https://crates.io/api/v1/crates/downcast-rs/1.2.0/download",
        type = "tar.gz",
        sha256 = "9ea835d29036a4087793836fa931b08837ad5e957da9e23886b29586fb9b6650",
        strip_prefix = "downcast-rs-1.2.0",
        build_file = Label("//cargo/remote:BUILD.downcast-rs-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fdeflate__0_3_0",
        url = "https://crates.io/api/v1/crates/fdeflate/0.3.0/download",
        type = "tar.gz",
        sha256 = "d329bdeac514ee06249dabc27877490f17f5d371ec693360768b838e19f3ae10",
        strip_prefix = "fdeflate-0.3.0",
        build_file = Label("//cargo/remote:BUILD.fdeflate-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__flate2__1_0_26",
        url = "https://crates.io/api/v1/crates/flate2/1.0.26/download",
        type = "tar.gz",
        sha256 = "3b9429470923de8e8cbd4d2dc513535400b4b3fef0319fb5c4e1f520a7bef743",
        strip_prefix = "flate2-1.0.26",
        build_file = Label("//cargo/remote:BUILD.flate2-1.0.26.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__foreign_types__0_3_2",
        url = "https://crates.io/api/v1/crates/foreign-types/0.3.2/download",
        type = "tar.gz",
        sha256 = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1",
        strip_prefix = "foreign-types-0.3.2",
        build_file = Label("//cargo/remote:BUILD.foreign-types-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__foreign_types_shared__0_1_1",
        url = "https://crates.io/api/v1/crates/foreign-types-shared/0.1.1/download",
        type = "tar.gz",
        sha256 = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b",
        strip_prefix = "foreign-types-shared-0.1.1",
        build_file = Label("//cargo/remote:BUILD.foreign-types-shared-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__gl_generator__0_14_0",
        url = "https://crates.io/api/v1/crates/gl_generator/0.14.0/download",
        type = "tar.gz",
        sha256 = "1a95dfc23a2b4a9a2f5ab41d194f8bfda3cabec42af4e39f08c339eb2a0c124d",
        strip_prefix = "gl_generator-0.14.0",
        build_file = Label("//cargo/remote:BUILD.gl_generator-0.14.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__glutin__0_30_9",
        url = "https://crates.io/api/v1/crates/glutin/0.30.9/download",
        type = "tar.gz",
        sha256 = "23b0385782048be65f0a9dd046c469d6a758a53fe1aa63a8111dea394d2ffa2f",
        strip_prefix = "glutin-0.30.9",
        build_file = Label("//cargo/remote:BUILD.glutin-0.30.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__glutin_winit__0_3_0",
        url = "https://crates.io/api/v1/crates/glutin-winit/0.3.0/download",
        type = "tar.gz",
        sha256 = "629a873fc04062830bfe8f97c03773bcd7b371e23bcc465d0a61448cd1588fa4",
        strip_prefix = "glutin-winit-0.3.0",
        build_file = Label("//cargo/remote:BUILD.glutin-winit-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__glutin_egl_sys__0_5_0",
        url = "https://crates.io/api/v1/crates/glutin_egl_sys/0.5.0/download",
        type = "tar.gz",
        sha256 = "1b3bcbddc51573b977fc6dca5d93867e4f29682cdbaf5d13e48f4fa4346d4d87",
        strip_prefix = "glutin_egl_sys-0.5.0",
        build_file = Label("//cargo/remote:BUILD.glutin_egl_sys-0.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__glutin_glx_sys__0_4_0",
        url = "https://crates.io/api/v1/crates/glutin_glx_sys/0.4.0/download",
        type = "tar.gz",
        sha256 = "1b53cb5fe568964aa066a3ba91eac5ecbac869fb0842cd0dc9e412434f1a1494",
        strip_prefix = "glutin_glx_sys-0.4.0",
        build_file = Label("//cargo/remote:BUILD.glutin_glx_sys-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__glutin_wgl_sys__0_4_0",
        url = "https://crates.io/api/v1/crates/glutin_wgl_sys/0.4.0/download",
        type = "tar.gz",
        sha256 = "ef89398e90033fc6bc65e9bd42fd29bbbfd483bda5b56dc5562f455550618165",
        strip_prefix = "glutin_wgl_sys-0.4.0",
        build_file = Label("//cargo/remote:BUILD.glutin_wgl_sys-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hashbrown__0_12_3",
        url = "https://crates.io/api/v1/crates/hashbrown/0.12.3/download",
        type = "tar.gz",
        sha256 = "8a9ee70c43aaf417c914396645a0fa852624801b24ebb7ae78fe8272889ac888",
        strip_prefix = "hashbrown-0.12.3",
        build_file = Label("//cargo/remote:BUILD.hashbrown-0.12.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__indexmap__1_9_3",
        url = "https://crates.io/api/v1/crates/indexmap/1.9.3/download",
        type = "tar.gz",
        sha256 = "bd070e393353796e801d209ad339e89596eb4c8d430d18ede6a1cced8fafbd99",
        strip_prefix = "indexmap-1.9.3",
        build_file = Label("//cargo/remote:BUILD.indexmap-1.9.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__instant__0_1_12",
        url = "https://crates.io/api/v1/crates/instant/0.1.12/download",
        type = "tar.gz",
        sha256 = "7a5bbe824c507c5da5956355e86a746d82e0e1464f65d862cc5e71da70e94b2c",
        strip_prefix = "instant-0.1.12",
        build_file = Label("//cargo/remote:BUILD.instant-0.1.12.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__jni_sys__0_3_0",
        url = "https://crates.io/api/v1/crates/jni-sys/0.3.0/download",
        type = "tar.gz",
        sha256 = "8eaf4bc02d17cbdd7ff4c7438cafcdf7fb9a4613313ad11b4f8fefe7d3fa0130",
        strip_prefix = "jni-sys-0.3.0",
        build_file = Label("//cargo/remote:BUILD.jni-sys-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__jobserver__0_1_26",
        url = "https://crates.io/api/v1/crates/jobserver/0.1.26/download",
        type = "tar.gz",
        sha256 = "936cfd212a0155903bcbc060e316fb6cc7cbf2e1907329391ebadc1fe0ce77c2",
        strip_prefix = "jobserver-0.1.26",
        build_file = Label("//cargo/remote:BUILD.jobserver-0.1.26.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__js_sys__0_3_64",
        url = "https://crates.io/api/v1/crates/js-sys/0.3.64/download",
        type = "tar.gz",
        sha256 = "c5f195fe497f702db0f318b07fdd68edb16955aed830df8363d837542f8f935a",
        strip_prefix = "js-sys-0.3.64",
        build_file = Label("//cargo/remote:BUILD.js-sys-0.3.64.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__khronos_api__3_1_0",
        url = "https://crates.io/api/v1/crates/khronos_api/3.1.0/download",
        type = "tar.gz",
        sha256 = "e2db585e1d738fc771bf08a151420d3ed193d9d895a36df7f6f8a9456b911ddc",
        strip_prefix = "khronos_api-3.1.0",
        build_file = Label("//cargo/remote:BUILD.khronos_api-3.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libc__0_2_146",
        url = "https://crates.io/api/v1/crates/libc/0.2.146/download",
        type = "tar.gz",
        sha256 = "f92be4933c13fd498862a9e02a3055f8a8d9c039ce33db97306fd5a6caa7f29b",
        strip_prefix = "libc-0.2.146",
        build_file = Label("//cargo/remote:BUILD.libc-0.2.146.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libloading__0_7_4",
        url = "https://crates.io/api/v1/crates/libloading/0.7.4/download",
        type = "tar.gz",
        sha256 = "b67380fd3b2fbe7527a606e18729d21c6f3951633d0500574c4dc22d2d638b9f",
        strip_prefix = "libloading-0.7.4",
        build_file = Label("//cargo/remote:BUILD.libloading-0.7.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libloading__0_8_0",
        url = "https://crates.io/api/v1/crates/libloading/0.8.0/download",
        type = "tar.gz",
        sha256 = "d580318f95776505201b28cf98eb1fa5e4be3b689633ba6a3e6cd880ff22d8cb",
        strip_prefix = "libloading-0.8.0",
        build_file = Label("//cargo/remote:BUILD.libloading-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_19",
        url = "https://crates.io/api/v1/crates/log/0.4.19/download",
        type = "tar.gz",
        sha256 = "b06a4cde4c0f271a446782e3eff8de789548ce57dbc8eca9292c27f4a42004b4",
        strip_prefix = "log-0.4.19",
        build_file = Label("//cargo/remote:BUILD.log-0.4.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memchr__2_5_0",
        url = "https://crates.io/api/v1/crates/memchr/2.5.0/download",
        type = "tar.gz",
        sha256 = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d",
        strip_prefix = "memchr-2.5.0",
        build_file = Label("//cargo/remote:BUILD.memchr-2.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memmap2__0_5_10",
        url = "https://crates.io/api/v1/crates/memmap2/0.5.10/download",
        type = "tar.gz",
        sha256 = "83faa42c0a078c393f6b29d5db232d8be22776a891f8f56e5284faee4a20b327",
        strip_prefix = "memmap2-0.5.10",
        build_file = Label("//cargo/remote:BUILD.memmap2-0.5.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memoffset__0_6_5",
        url = "https://crates.io/api/v1/crates/memoffset/0.6.5/download",
        type = "tar.gz",
        sha256 = "5aa361d4faea93603064a027415f07bd8e1d5c88c9fbf68bf56a285428fd79ce",
        strip_prefix = "memoffset-0.6.5",
        build_file = Label("//cargo/remote:BUILD.memoffset-0.6.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__minimal_lexical__0_2_1",
        url = "https://crates.io/api/v1/crates/minimal-lexical/0.2.1/download",
        type = "tar.gz",
        sha256 = "68354c5c6bd36d73ff3feceb05efa59b6acb7626617f4962be322a825e61f79a",
        strip_prefix = "minimal-lexical-0.2.1",
        build_file = Label("//cargo/remote:BUILD.minimal-lexical-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__miniz_oxide__0_7_1",
        url = "https://crates.io/api/v1/crates/miniz_oxide/0.7.1/download",
        type = "tar.gz",
        sha256 = "e7810e0be55b428ada41041c41f32c9f1a42817901b4ccf45fa3d4b6561e74c7",
        strip_prefix = "miniz_oxide-0.7.1",
        build_file = Label("//cargo/remote:BUILD.miniz_oxide-0.7.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__mio__0_8_8",
        url = "https://crates.io/api/v1/crates/mio/0.8.8/download",
        type = "tar.gz",
        sha256 = "927a765cd3fc26206e66b296465fa9d3e5ab003e651c1b3c060e7956d96b19d2",
        strip_prefix = "mio-0.8.8",
        build_file = Label("//cargo/remote:BUILD.mio-0.8.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk__0_7_0",
        url = "https://crates.io/api/v1/crates/ndk/0.7.0/download",
        type = "tar.gz",
        sha256 = "451422b7e4718271c8b5b3aadf5adedba43dc76312454b387e98fae0fc951aa0",
        strip_prefix = "ndk-0.7.0",
        build_file = Label("//cargo/remote:BUILD.ndk-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk_context__0_1_1",
        url = "https://crates.io/api/v1/crates/ndk-context/0.1.1/download",
        type = "tar.gz",
        sha256 = "27b02d87554356db9e9a873add8782d4ea6e3e58ea071a9adb9a2e8ddb884a8b",
        strip_prefix = "ndk-context-0.1.1",
        build_file = Label("//cargo/remote:BUILD.ndk-context-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk_sys__0_4_1_23_1_7779620",
        url = "https://crates.io/api/v1/crates/ndk-sys/0.4.1+23.1.7779620/download",
        type = "tar.gz",
        sha256 = "3cf2aae958bd232cac5069850591667ad422d263686d75b52a065f9badeee5a3",
        strip_prefix = "ndk-sys-0.4.1+23.1.7779620",
        build_file = Label("//cargo/remote:BUILD.ndk-sys-0.4.1+23.1.7779620.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nix__0_24_3",
        url = "https://crates.io/api/v1/crates/nix/0.24.3/download",
        type = "tar.gz",
        sha256 = "fa52e972a9a719cecb6864fb88568781eb706bac2cd1d4f04a648542dbf78069",
        strip_prefix = "nix-0.24.3",
        build_file = Label("//cargo/remote:BUILD.nix-0.24.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nix__0_25_1",
        url = "https://crates.io/api/v1/crates/nix/0.25.1/download",
        type = "tar.gz",
        sha256 = "f346ff70e7dbfd675fe90590b92d59ef2de15a8779ae305ebcbfd3f0caf59be4",
        strip_prefix = "nix-0.25.1",
        build_file = Label("//cargo/remote:BUILD.nix-0.25.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nom__7_1_3",
        url = "https://crates.io/api/v1/crates/nom/7.1.3/download",
        type = "tar.gz",
        sha256 = "d273983c5a657a70a3e8f2a01329822f3b8c8172b73826411a55751e404a0a4a",
        strip_prefix = "nom-7.1.3",
        build_file = Label("//cargo/remote:BUILD.nom-7.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_enum__0_5_11",
        url = "https://crates.io/api/v1/crates/num_enum/0.5.11/download",
        type = "tar.gz",
        sha256 = "1f646caf906c20226733ed5b1374287eb97e3c2a5c227ce668c1f2ce20ae57c9",
        strip_prefix = "num_enum-0.5.11",
        build_file = Label("//cargo/remote:BUILD.num_enum-0.5.11.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_enum_derive__0_5_11",
        url = "https://crates.io/api/v1/crates/num_enum_derive/0.5.11/download",
        type = "tar.gz",
        sha256 = "dcbff9bc912032c62bf65ef1d5aea88983b420f4f839db1e9b0c281a25c9c799",
        strip_prefix = "num_enum_derive-0.5.11",
        build_file = Label("//cargo/remote:BUILD.num_enum_derive-0.5.11.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__objc_sys__0_2_0_beta_2",
        url = "https://crates.io/api/v1/crates/objc-sys/0.2.0-beta.2/download",
        type = "tar.gz",
        sha256 = "df3b9834c1e95694a05a828b59f55fa2afec6288359cda67146126b3f90a55d7",
        strip_prefix = "objc-sys-0.2.0-beta.2",
        build_file = Label("//cargo/remote:BUILD.objc-sys-0.2.0-beta.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__objc2__0_3_0_beta_3_patch_leaks_3",
        url = "https://crates.io/api/v1/crates/objc2/0.3.0-beta.3.patch-leaks.3/download",
        type = "tar.gz",
        sha256 = "7e01640f9f2cb1220bbe80325e179e532cb3379ebcd1bf2279d703c19fe3a468",
        strip_prefix = "objc2-0.3.0-beta.3.patch-leaks.3",
        build_file = Label("//cargo/remote:BUILD.objc2-0.3.0-beta.3.patch-leaks.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__objc2_encode__2_0_0_pre_2",
        url = "https://crates.io/api/v1/crates/objc2-encode/2.0.0-pre.2/download",
        type = "tar.gz",
        sha256 = "abfcac41015b00a120608fdaa6938c44cb983fee294351cc4bac7638b4e50512",
        strip_prefix = "objc2-encode-2.0.0-pre.2",
        build_file = Label("//cargo/remote:BUILD.objc2-encode-2.0.0-pre.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__once_cell__1_18_0",
        url = "https://crates.io/api/v1/crates/once_cell/1.18.0/download",
        type = "tar.gz",
        sha256 = "dd8b5dd2ae5ed71462c540258bedcb51965123ad7e7ccf4b9a8cafaa4a63576d",
        strip_prefix = "once_cell-1.18.0",
        build_file = Label("//cargo/remote:BUILD.once_cell-1.18.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__orbclient__0_3_45",
        url = "https://crates.io/api/v1/crates/orbclient/0.3.45/download",
        type = "tar.gz",
        sha256 = "221d488cd70617f1bd599ed8ceb659df2147d9393717954d82a0f5e8032a6ab1",
        strip_prefix = "orbclient-0.3.45",
        build_file = Label("//cargo/remote:BUILD.orbclient-0.3.45.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__owned_ttf_parser__0_19_0",
        url = "https://crates.io/api/v1/crates/owned_ttf_parser/0.19.0/download",
        type = "tar.gz",
        sha256 = "706de7e2214113d63a8238d1910463cfce781129a6f263d13fdb09ff64355ba4",
        strip_prefix = "owned_ttf_parser-0.19.0",
        build_file = Label("//cargo/remote:BUILD.owned_ttf_parser-0.19.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__percent_encoding__2_3_0",
        url = "https://crates.io/api/v1/crates/percent-encoding/2.3.0/download",
        type = "tar.gz",
        sha256 = "9b2a4787296e9989611394c33f193f676704af1686e70b8f8033ab5ba9a35a94",
        strip_prefix = "percent-encoding-2.3.0",
        build_file = Label("//cargo/remote:BUILD.percent-encoding-2.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pkg_config__0_3_27",
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.27/download",
        type = "tar.gz",
        sha256 = "26072860ba924cbfa98ea39c8c19b4dd6a4a25423dbdf219c1eca91aa0cf6964",
        strip_prefix = "pkg-config-0.3.27",
        build_file = Label("//cargo/remote:BUILD.pkg-config-0.3.27.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__png__0_17_9",
        url = "https://crates.io/api/v1/crates/png/0.17.9/download",
        type = "tar.gz",
        sha256 = "59871cc5b6cce7eaccca5a802b4173377a1c2ba90654246789a8fa2334426d11",
        strip_prefix = "png-0.17.9",
        build_file = Label("//cargo/remote:BUILD.png-0.17.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro_crate__1_3_1",
        url = "https://crates.io/api/v1/crates/proc-macro-crate/1.3.1/download",
        type = "tar.gz",
        sha256 = "7f4c021e1093a56626774e81216a4ce732a735e5bad4868a03f3ed65ca0c3919",
        strip_prefix = "proc-macro-crate-1.3.1",
        build_file = Label("//cargo/remote:BUILD.proc-macro-crate-1.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_60",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.60/download",
        type = "tar.gz",
        sha256 = "dec2b086b7a862cf4de201096214fa870344cf922b2b30c167badb3af3195406",
        strip_prefix = "proc-macro2-1.0.60",
        build_file = Label("//cargo/remote:BUILD.proc-macro2-1.0.60.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_28",
        url = "https://crates.io/api/v1/crates/quote/1.0.28/download",
        type = "tar.gz",
        sha256 = "1b9ab9c7eadfd8df19006f1cf1a4aed13540ed5cbc047010ece5826e10825488",
        strip_prefix = "quote-1.0.28",
        build_file = Label("//cargo/remote:BUILD.quote-1.0.28.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__raw_window_handle__0_5_2",
        url = "https://crates.io/api/v1/crates/raw-window-handle/0.5.2/download",
        type = "tar.gz",
        sha256 = "f2ff9a1f06a88b01621b7ae906ef0211290d1c8a168a15542486a8f61c0833b9",
        strip_prefix = "raw-window-handle-0.5.2",
        build_file = Label("//cargo/remote:BUILD.raw-window-handle-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__redox_syscall__0_3_5",
        url = "https://crates.io/api/v1/crates/redox_syscall/0.3.5/download",
        type = "tar.gz",
        sha256 = "567664f262709473930a4bf9e51bf2ebf3348f2e748ccc50dea20646858f8f29",
        strip_prefix = "redox_syscall-0.3.5",
        build_file = Label("//cargo/remote:BUILD.redox_syscall-0.3.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__scoped_tls__1_0_1",
        url = "https://crates.io/api/v1/crates/scoped-tls/1.0.1/download",
        type = "tar.gz",
        sha256 = "e1cf6437eb19a8f4a6cc0f7dca544973b0b78843adbfeb3683d1a94a0024a294",
        strip_prefix = "scoped-tls-1.0.1",
        build_file = Label("//cargo/remote:BUILD.scoped-tls-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__sctk_adwaita__0_5_4",
        url = "https://crates.io/api/v1/crates/sctk-adwaita/0.5.4/download",
        type = "tar.gz",
        sha256 = "cda4e97be1fd174ccc2aae81c8b694e803fa99b34e8fd0f057a9d70698e3ed09",
        strip_prefix = "sctk-adwaita-0.5.4",
        build_file = Label("//cargo/remote:BUILD.sctk-adwaita-0.5.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__simd_adler32__0_3_5",
        url = "https://crates.io/api/v1/crates/simd-adler32/0.3.5/download",
        type = "tar.gz",
        sha256 = "238abfbb77c1915110ad968465608b68e869e0772622c9656714e73e5a1a522f",
        strip_prefix = "simd-adler32-0.3.5",
        build_file = Label("//cargo/remote:BUILD.simd-adler32-0.3.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slotmap__1_0_6",
        url = "https://crates.io/api/v1/crates/slotmap/1.0.6/download",
        type = "tar.gz",
        sha256 = "e1e08e261d0e8f5c43123b7adf3e4ca1690d655377ac93a03b2c9d3e98de1342",
        strip_prefix = "slotmap-1.0.6",
        build_file = Label("//cargo/remote:BUILD.slotmap-1.0.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__smallvec__1_10_0",
        url = "https://crates.io/api/v1/crates/smallvec/1.10.0/download",
        type = "tar.gz",
        sha256 = "a507befe795404456341dfab10cef66ead4c041f62b8b11bbb92bffe5d0953e0",
        strip_prefix = "smallvec-1.10.0",
        build_file = Label("//cargo/remote:BUILD.smallvec-1.10.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__smithay_client_toolkit__0_16_0",
        url = "https://crates.io/api/v1/crates/smithay-client-toolkit/0.16.0/download",
        type = "tar.gz",
        sha256 = "f307c47d32d2715eb2e0ece5589057820e0e5e70d07c247d1063e844e107f454",
        strip_prefix = "smithay-client-toolkit-0.16.0",
        build_file = Label("//cargo/remote:BUILD.smithay-client-toolkit-0.16.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__strict_num__0_1_1",
        url = "https://crates.io/api/v1/crates/strict-num/0.1.1/download",
        type = "tar.gz",
        sha256 = "6637bab7722d379c8b41ba849228d680cc12d0a45ba1fa2b48f2a30577a06731",
        strip_prefix = "strict-num-0.1.1",
        build_file = Label("//cargo/remote:BUILD.strict-num-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_109",
        url = "https://crates.io/api/v1/crates/syn/1.0.109/download",
        type = "tar.gz",
        sha256 = "72b64191b275b66ffe2469e8af2c1cfe3bafa67b529ead792a6d0160888b4237",
        strip_prefix = "syn-1.0.109",
        build_file = Label("//cargo/remote:BUILD.syn-1.0.109.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__2_0_18",
        url = "https://crates.io/api/v1/crates/syn/2.0.18/download",
        type = "tar.gz",
        sha256 = "32d41677bcbe24c20c52e7c70b0d8db04134c5d1066bf98662e2871ad200ea3e",
        strip_prefix = "syn-2.0.18",
        build_file = Label("//cargo/remote:BUILD.syn-2.0.18.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror__1_0_40",
        url = "https://crates.io/api/v1/crates/thiserror/1.0.40/download",
        type = "tar.gz",
        sha256 = "978c9a314bd8dc99be594bc3c175faaa9794be04a5a5e153caba6915336cebac",
        strip_prefix = "thiserror-1.0.40",
        build_file = Label("//cargo/remote:BUILD.thiserror-1.0.40.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror_impl__1_0_40",
        url = "https://crates.io/api/v1/crates/thiserror-impl/1.0.40/download",
        type = "tar.gz",
        sha256 = "f9456a42c5b0d803c8cd86e73dd7cc9edd429499f37a3550d286d5e86720569f",
        strip_prefix = "thiserror-impl-1.0.40",
        build_file = Label("//cargo/remote:BUILD.thiserror-impl-1.0.40.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tiny_skia__0_8_4",
        url = "https://crates.io/api/v1/crates/tiny-skia/0.8.4/download",
        type = "tar.gz",
        sha256 = "df8493a203431061e901613751931f047d1971337153f96d0e5e363d6dbf6a67",
        strip_prefix = "tiny-skia-0.8.4",
        build_file = Label("//cargo/remote:BUILD.tiny-skia-0.8.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tiny_skia_path__0_8_4",
        url = "https://crates.io/api/v1/crates/tiny-skia-path/0.8.4/download",
        type = "tar.gz",
        sha256 = "adbfb5d3f3dd57a0e11d12f4f13d4ebbbc1b5c15b7ab0a156d030b21da5f677c",
        strip_prefix = "tiny-skia-path-0.8.4",
        build_file = Label("//cargo/remote:BUILD.tiny-skia-path-0.8.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__toml_datetime__0_6_2",
        url = "https://crates.io/api/v1/crates/toml_datetime/0.6.2/download",
        type = "tar.gz",
        sha256 = "5a76a9312f5ba4c2dec6b9161fdf25d87ad8a09256ccea5a556fef03c706a10f",
        strip_prefix = "toml_datetime-0.6.2",
        build_file = Label("//cargo/remote:BUILD.toml_datetime-0.6.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__toml_edit__0_19_10",
        url = "https://crates.io/api/v1/crates/toml_edit/0.19.10/download",
        type = "tar.gz",
        sha256 = "2380d56e8670370eee6566b0bfd4265f65b3f432e8c6d85623f728d4fa31f739",
        strip_prefix = "toml_edit-0.19.10",
        build_file = Label("//cargo/remote:BUILD.toml_edit-0.19.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ttf_parser__0_19_1",
        url = "https://crates.io/api/v1/crates/ttf-parser/0.19.1/download",
        type = "tar.gz",
        sha256 = "a464a4b34948a5f67fddd2b823c62d9d92e44be75058b99939eae6c5b6960b33",
        strip_prefix = "ttf-parser-0.19.1",
        build_file = Label("//cargo/remote:BUILD.ttf-parser-0.19.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_ident__1_0_9",
        url = "https://crates.io/api/v1/crates/unicode-ident/1.0.9/download",
        type = "tar.gz",
        sha256 = "b15811caf2415fb889178633e7724bad2509101cde276048e013b9def5e51fa0",
        strip_prefix = "unicode-ident-1.0.9",
        build_file = Label("//cargo/remote:BUILD.unicode-ident-1.0.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__vec_map__0_8_2",
        url = "https://crates.io/api/v1/crates/vec_map/0.8.2/download",
        type = "tar.gz",
        sha256 = "f1bddf1187be692e79c5ffeab891132dfb0f236ed36a43c7ed39f1165ee20191",
        strip_prefix = "vec_map-0.8.2",
        build_file = Label("//cargo/remote:BUILD.vec_map-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__version_check__0_9_4",
        url = "https://crates.io/api/v1/crates/version_check/0.9.4/download",
        type = "tar.gz",
        sha256 = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f",
        strip_prefix = "version_check-0.9.4",
        build_file = Label("//cargo/remote:BUILD.version_check-0.9.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_11_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.11.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423",
        strip_prefix = "wasi-0.11.0+wasi-snapshot-preview1",
        build_file = Label("//cargo/remote:BUILD.wasi-0.11.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen__0_2_87",
        url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.87/download",
        type = "tar.gz",
        sha256 = "7706a72ab36d8cb1f80ffbf0e071533974a60d0a308d01a5d0375bf60499a342",
        strip_prefix = "wasm-bindgen-0.2.87",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-0.2.87.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_backend__0_2_87",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-backend/0.2.87/download",
        type = "tar.gz",
        sha256 = "5ef2b6d3c510e9625e5fe6f509ab07d66a760f0885d858736483c32ed7809abd",
        strip_prefix = "wasm-bindgen-backend-0.2.87",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-backend-0.2.87.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro__0_2_87",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.87/download",
        type = "tar.gz",
        sha256 = "dee495e55982a3bd48105a7b947fd2a9b4a8ae3010041b9e0faab3f9cd028f1d",
        strip_prefix = "wasm-bindgen-macro-0.2.87",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-0.2.87.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro_support__0_2_87",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.87/download",
        type = "tar.gz",
        sha256 = "54681b18a46765f095758388f2d0cf16eb8d4169b639ab575a8f5693af210c7b",
        strip_prefix = "wasm-bindgen-macro-support-0.2.87",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-support-0.2.87.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_shared__0_2_87",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.87/download",
        type = "tar.gz",
        sha256 = "ca6ad05a4870b2bf5fe995117d3728437bd27d7cd5f06f13c17443ef369775a1",
        strip_prefix = "wasm-bindgen-shared-0.2.87",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-shared-0.2.87.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_client__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-client/0.29.5/download",
        type = "tar.gz",
        sha256 = "3f3b068c05a039c9f755f881dc50f01732214f5685e379829759088967c46715",
        strip_prefix = "wayland-client-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-client-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_commons__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-commons/0.29.5/download",
        type = "tar.gz",
        sha256 = "8691f134d584a33a6606d9d717b95c4fa20065605f798a3f350d78dced02a902",
        strip_prefix = "wayland-commons-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-commons-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_cursor__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-cursor/0.29.5/download",
        type = "tar.gz",
        sha256 = "6865c6b66f13d6257bef1cd40cbfe8ef2f150fb8ebbdb1e8e873455931377661",
        strip_prefix = "wayland-cursor-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-cursor-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_protocols__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-protocols/0.29.5/download",
        type = "tar.gz",
        sha256 = "b950621f9354b322ee817a23474e479b34be96c2e909c14f7bc0100e9a970bc6",
        strip_prefix = "wayland-protocols-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-protocols-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_scanner__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-scanner/0.29.5/download",
        type = "tar.gz",
        sha256 = "8f4303d8fa22ab852f789e75a967f0a2cdc430a607751c0499bada3e451cbd53",
        strip_prefix = "wayland-scanner-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-scanner-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_sys__0_29_5",
        url = "https://crates.io/api/v1/crates/wayland-sys/0.29.5/download",
        type = "tar.gz",
        sha256 = "be12ce1a3c39ec7dba25594b97b42cb3195d54953ddb9d3d95a7c3902bc6e9d4",
        strip_prefix = "wayland-sys-0.29.5",
        build_file = Label("//cargo/remote:BUILD.wayland-sys-0.29.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_sys__0_30_1",
        url = "https://crates.io/api/v1/crates/wayland-sys/0.30.1/download",
        type = "tar.gz",
        sha256 = "96b2a02ac608e07132978689a6f9bf4214949c85998c247abadd4f4129b1aa06",
        strip_prefix = "wayland-sys-0.30.1",
        build_file = Label("//cargo/remote:BUILD.wayland-sys-0.30.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__web_sys__0_3_64",
        url = "https://crates.io/api/v1/crates/web-sys/0.3.64/download",
        type = "tar.gz",
        sha256 = "9b85cbef8c220a6abc02aefd892dfc0fc23afb1c6a426316ec33253a3877249b",
        strip_prefix = "web-sys-0.3.64",
        build_file = Label("//cargo/remote:BUILD.web-sys-0.3.64.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_sys__0_45_0",
        url = "https://crates.io/api/v1/crates/windows-sys/0.45.0/download",
        type = "tar.gz",
        sha256 = "75283be5efb2831d37ea142365f009c02ec203cd29a3ebecbc093d52315b66d0",
        strip_prefix = "windows-sys-0.45.0",
        build_file = Label("//cargo/remote:BUILD.windows-sys-0.45.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_sys__0_48_0",
        url = "https://crates.io/api/v1/crates/windows-sys/0.48.0/download",
        type = "tar.gz",
        sha256 = "677d2418bec65e3338edb076e806bc1ec15693c5d0104683f2efe857f61056a9",
        strip_prefix = "windows-sys-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows-sys-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_targets__0_42_2",
        url = "https://crates.io/api/v1/crates/windows-targets/0.42.2/download",
        type = "tar.gz",
        sha256 = "8e5180c00cd44c9b1c88adb3693291f1cd93605ded80c250a75d472756b4d071",
        strip_prefix = "windows-targets-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows-targets-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_targets__0_48_0",
        url = "https://crates.io/api/v1/crates/windows-targets/0.48.0/download",
        type = "tar.gz",
        sha256 = "7b1eb6f0cd7c80c79759c929114ef071b87354ce476d9d94271031c0497adfd5",
        strip_prefix = "windows-targets-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows-targets-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_aarch64_gnullvm__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_aarch64_gnullvm/0.42.2/download",
        type = "tar.gz",
        sha256 = "597a5118570b68bc08d8d59125332c54f1ba9d9adeedeef5b99b02ba2b0698f8",
        strip_prefix = "windows_aarch64_gnullvm-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_aarch64_gnullvm-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_aarch64_gnullvm__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_aarch64_gnullvm/0.48.0/download",
        type = "tar.gz",
        sha256 = "91ae572e1b79dba883e0d315474df7305d12f569b400fcf90581b06062f7e1bc",
        strip_prefix = "windows_aarch64_gnullvm-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_aarch64_gnullvm-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_aarch64_msvc__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_aarch64_msvc/0.42.2/download",
        type = "tar.gz",
        sha256 = "e08e8864a60f06ef0d0ff4ba04124db8b0fb3be5776a5cd47641e942e58c4d43",
        strip_prefix = "windows_aarch64_msvc-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_aarch64_msvc-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_aarch64_msvc__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_aarch64_msvc/0.48.0/download",
        type = "tar.gz",
        sha256 = "b2ef27e0d7bdfcfc7b868b317c1d32c641a6fe4629c171b8928c7b08d98d7cf3",
        strip_prefix = "windows_aarch64_msvc-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_aarch64_msvc-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_i686_gnu__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_i686_gnu/0.42.2/download",
        type = "tar.gz",
        sha256 = "c61d927d8da41da96a81f029489353e68739737d3beca43145c8afec9a31a84f",
        strip_prefix = "windows_i686_gnu-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_i686_gnu-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_i686_gnu__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_i686_gnu/0.48.0/download",
        type = "tar.gz",
        sha256 = "622a1962a7db830d6fd0a69683c80a18fda201879f0f447f065a3b7467daa241",
        strip_prefix = "windows_i686_gnu-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_i686_gnu-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_i686_msvc__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_i686_msvc/0.42.2/download",
        type = "tar.gz",
        sha256 = "44d840b6ec649f480a41c8d80f9c65108b92d89345dd94027bfe06ac444d1060",
        strip_prefix = "windows_i686_msvc-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_i686_msvc-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_i686_msvc__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_i686_msvc/0.48.0/download",
        type = "tar.gz",
        sha256 = "4542c6e364ce21bf45d69fdd2a8e455fa38d316158cfd43b3ac1c5b1b19f8e00",
        strip_prefix = "windows_i686_msvc-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_i686_msvc-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_gnu__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_x86_64_gnu/0.42.2/download",
        type = "tar.gz",
        sha256 = "8de912b8b8feb55c064867cf047dda097f92d51efad5b491dfb98f6bbb70cb36",
        strip_prefix = "windows_x86_64_gnu-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_gnu-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_gnu__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_x86_64_gnu/0.48.0/download",
        type = "tar.gz",
        sha256 = "ca2b8a661f7628cbd23440e50b05d705db3686f894fc9580820623656af974b1",
        strip_prefix = "windows_x86_64_gnu-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_gnu-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_gnullvm__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_x86_64_gnullvm/0.42.2/download",
        type = "tar.gz",
        sha256 = "26d41b46a36d453748aedef1486d5c7a85db22e56aff34643984ea85514e94a3",
        strip_prefix = "windows_x86_64_gnullvm-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_gnullvm-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_gnullvm__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_x86_64_gnullvm/0.48.0/download",
        type = "tar.gz",
        sha256 = "7896dbc1f41e08872e9d5e8f8baa8fdd2677f29468c4e156210174edc7f7b953",
        strip_prefix = "windows_x86_64_gnullvm-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_gnullvm-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_msvc__0_42_2",
        url = "https://crates.io/api/v1/crates/windows_x86_64_msvc/0.42.2/download",
        type = "tar.gz",
        sha256 = "9aec5da331524158c6d1a4ac0ab1541149c0b9505fde06423b02f5ef0106b9f0",
        strip_prefix = "windows_x86_64_msvc-0.42.2",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_msvc-0.42.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__windows_x86_64_msvc__0_48_0",
        url = "https://crates.io/api/v1/crates/windows_x86_64_msvc/0.48.0/download",
        type = "tar.gz",
        sha256 = "1a515f5799fe4961cb532f983ce2b23082366b898e52ffbce459c86f67c8378a",
        strip_prefix = "windows_x86_64_msvc-0.48.0",
        build_file = Label("//cargo/remote:BUILD.windows_x86_64_msvc-0.48.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winit__0_28_6",
        url = "https://crates.io/api/v1/crates/winit/0.28.6/download",
        type = "tar.gz",
        sha256 = "866db3f712fffba75d31bf0cdecf357c8aeafd158c5b7ab51dba2a2b2d47f196",
        strip_prefix = "winit-0.28.6",
        build_file = Label("//cargo/remote:BUILD.winit-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winnow__0_4_7",
        url = "https://crates.io/api/v1/crates/winnow/0.4.7/download",
        type = "tar.gz",
        sha256 = "ca0ace3845f0d96209f0375e6d367e3eb87eb65d27d445bdc9f1843a26f39448",
        strip_prefix = "winnow-0.4.7",
        build_file = Label("//cargo/remote:BUILD.winnow-0.4.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__x11_dl__2_21_0",
        url = "https://crates.io/api/v1/crates/x11-dl/2.21.0/download",
        type = "tar.gz",
        sha256 = "38735924fedd5314a6e548792904ed8c6de6636285cb9fec04d5b1db85c1516f",
        strip_prefix = "x11-dl-2.21.0",
        build_file = Label("//cargo/remote:BUILD.x11-dl-2.21.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__xcursor__0_3_4",
        url = "https://crates.io/api/v1/crates/xcursor/0.3.4/download",
        type = "tar.gz",
        sha256 = "463705a63313cd4301184381c5e8042f0a7e9b4bb63653f216311d4ae74690b7",
        strip_prefix = "xcursor-0.3.4",
        build_file = Label("//cargo/remote:BUILD.xcursor-0.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__xml_rs__0_8_14",
        url = "https://crates.io/api/v1/crates/xml-rs/0.8.14/download",
        type = "tar.gz",
        sha256 = "52839dc911083a8ef63efa4d039d1f58b5e409f923e44c80828f206f66e5541c",
        strip_prefix = "xml-rs-0.8.14",
        build_file = Label("//cargo/remote:BUILD.xml-rs-0.8.14.bazel"),
    )
