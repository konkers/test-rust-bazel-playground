# Bazel Embedded Rust Playground

This repository is an experiment in building bare metal rust targets with Bazel.

## embedded-qemu

This experiment is adapted from the
[QEMU example](https://docs.rust-embedded.org/book/start/qemu.html) in
[The Embedded Rust Book](https://docs.rust-embedded.org/book/).

### Building

The binary can be built for the lm3s6965evb platform using:
```bash
> bazel build //embedded-qemu:qemu-hello --config qemu       
```

The resulting elf binary will end up in `//bazel-bin/embedded-qemu/qemu-hello`.

### Running

To run `qemu-hello` you need `qemu-system-arm` installed on your system.  One 
easy way of getting this is to bootstrap a
[Pigweed Environment](https://pigweed.dev/) which will download and install
QEMU and add it to your path.

`qemu-hello` can be run with the command:

```bash
> qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel ./bazel-bin/embedded-qemu/qemu-hello
```

You should see the following output:
```
Timer with period zero, disabling
Hello, world!
```

## Bazel Details / Findings

### WORKSPACE.bazel

In this file we pull in an embedded C++ tool chain as well as Rust support
using [rules_rust](https://github.com/bazelbuild/rules_rust).  `rules_rust`
is used to set up several things:

- A toolchain using `rust_repository_set`.
- A repository of third party crates using `crates_repository`.
- Support for rust-analyzer using `rust_analyzer_dependencies`.

Unfortunately due to how `bazel_embedded` specifies it's constraints, the
toolchains registered through `rules_rust` are not compatible with
[platforms](https://bazel.build/extending/platforms) set up for
`bazel_embedded`.  To work around this, we create a new toolchain based
on the one declared here in `//toolchain:rust_linux_x86_64`.

### //toolchain/BUILD.bazel

Here we declare a new toolchain which references the one created by
`rules_rust`.  This allows us to add additional constraints to be
compatible with platforms defined in terms of the `bazel_embedded`
constraints.  Specifically:

```
target_compatible_with = [
    "@platforms//cpu:armv7-m",
    "@bazel_embedded//constraints/fpu:none",
    "@platforms//os:none",
],
```

This toolchain is then registered in `//WORKSPACE.bazel`.

### //platforms/BUILD.bazel

Here we define an `lm3s6965evb` platform with the necessary constraints to
build binaries for the QEMU lm3s6965evb machine.

### //embedded-qemu/BUILD.bazel

Defines the `qemu-hello` target as a mostly standard `rust_binary`.  Of note
here is that we specify a linker script (`link.x`) as well as including 
`memory.x` (included from `link.x`) as a data dependency.

### //.bazelrc

Specifies default bazel arguments/configs

### build --incompatible_enable_cc_toolchain_resolution

This option is necessary to use Bazel platforms to resolve C/C++ toolchains.
`rules_rust` uses the C/C++ linker for the for the final link down.  Without
this option, `/bin/gcc` is used for that instead of the correctly compiler
for the target.

### build:qemu --platforms //platforms:lm3s6965evb

Sets up a convenient configuration alias for options needed to produce QEMU
binaries.  This is what makes `bazel build --config qemu` work.

## Rust Analyzer

To create a `rust-project.json` for rust-analyzer run:

```shell
> bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

For more information including VSCode support see `rules_rust`'s
[Rust analyzer documentation](https://bazelbuild.github.io/rules_rust).
