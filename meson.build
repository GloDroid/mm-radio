project(
  'mm-radio',
  ['c', 'cpp', 'rust'],
  version : '',
  license : 'APACHE-2.0',
  meson_version : '>= 0.56',
  default_options : ['buildtype=debugoptimized', 'b_ndebug=if-release', 'c_std=c11', 'cpp_std=c++17', 'cpp_rtti=false', 'rust_std=2021']
)

include_dirs = [include_directories('src/')]

srcs = []

subdir('src')
subdir('rust-dep')

deps = [
    dependency('android_base'),
    dependency('android_binder_ndk'),
    dependency('android_cutils'),
    dependency('android_log'),
    dependency('android_utils'),
    dependency('android.hardware.radio-V1-ndk'),
    dependency('android.hardware.radio.config-V1-ndk'),
    dependency('android.hardware.radio.data-V1-ndk'),
    dependency('android.hardware.radio.messaging-V1-ndk'),
    dependency('android.hardware.radio.modem-V1-ndk'),
    dependency('android.hardware.radio.network-V1-ndk'),
    dependency('android.hardware.radio.sim-V1-ndk'),
    dependency('android.hardware.radio.voice-V1-ndk'),
    dependency('glib-2.0'),
    dependency('gmodule-2.0'),
    dependency('gobject-2.0'),
    dependency('mm-glib'),
]

executable(
    'android.hardware.mm-radio-service',
    srcs,
    dependencies : deps,
    install : true,
    install_dir : get_option('bindir') / 'hw',
    include_directories: include_dirs,
    link_with: [libutils],
)

configure_file(
    input: 'android.hardware.radio.rc',
    output: '@PLAINNAME@',
    copy: true,
    install_dir: get_option('sysconfdir') / 'init',
)

configure_file(
  input: 'android.hardware.radio.xml',
  output: '@PLAINNAME@',
  copy: true,
  install_dir: get_option('sysconfdir') / 'vintf' / 'manifest',
)
