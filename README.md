This README.md is a copy of my blogpost at modio.se about rust cross compiling.

--- 

A common occurence when working with embedded systems is cross-compiling
source code from a larger system in order to build code that can execute on a
smaller one.

With [Rust](https://www.rust-lang.org/) this is usually all handled by  [Cargo](https://doc.rust-lang.org/stable/cargo/),
by adding the `--target` option when compiling your code.
So if you normally do `cargo build` you instead do `cargo build --target armv7-unknown-linux-gnueabihf`.

However, sometimes it's not all that simple. The machine you're compiling on
will need a [Cross compiler](https://en.wikipedia.org/wiki/Cross_compiler) as
well as copies of the target platform libraries. 

Common solutions for this are [Buildroot](https://buildroot.org/),
[Yocto](https://www.yoctoproject.org/), but here I'm going to cover how to use
[Debian](https://wiki.debian.org/CrossCompiling)  for this, as it's easy to use
in automated CI systems in containers.

If you don't have external or library dependencies, the easy tool is to use
[Rust Cross](https://github.com/rust-embedded/cross) from the rust embedded
team. If it works for you, stop there. In my linked EXAMPLE, the `cross` tool
works on the first example, as it has no system dependencies, while the second
example does not compile.

    ::shell
    cargo install cross
    cross build --target armv7-unknown-linux-gnueabihf

If it's not enough to use `cross`, for example if you want to run it all inside
a container, or in a CI system, or if you need other, external libraries,  here
are the steps I use to set up a Debian container to cross-compile rust code.

For a local desktop setup, adjust your path to not be `/cargo`, and remove the
`USER` variable as it should already be set.

    ::shell
    export CARGO_HOME=/cargo
    export PATH=/cargo/bin:$PATH
    export USER=root
    mkdir /cargo
    cat > /cargo/config << EOF
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    EOF

    dpkg --add-architecture armhf
    apt-get update 
    apt-get install -y curl git build-essential
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf
    apt-get install -y libdbus-1-dev libdbus-1-dev:armhf
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh
    bash /usr/local/bin/rustup.sh -y
    rustup default stable
    rustup target add x86_64-unknown-linux-gnu
    rustup target add armv7-unknown-linux-gnueabihf

    export PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
    export PKG_CONFIG_ALLOW_CROSS="true"

That's a lot to take in, so let's dig through it a bit, but first we need to
have a problem to solve.

I've prepared an [Rust cross
example](https://gitlab.com/Spindel/rust-cross-example/) here. Our goal is to
compile "cross_example_01" and "cross_example_02" from our moden x86_64
machines into binary code that can run on an Arm32 machine, like
[PocketBeagle](https://beagleboard.org/pocket).

For the [first
example](https://gitlab.com/Spindel/rust-cross-example/-/blob/master/cross_example_01/src/main.rs)
we just need to install the basic cross-compiler and it's all done.

The easiest way to do that with rust would be: 

    ::shell
    cargo install cross
    cross build --target armv7-unknown-linux-gnueabihf

And with the default setup of cross it will launch a `docker` container, spawn
the build inside that, and cross compile your code. If you want to use
[podman](https://podman.io/) instead of docker, you need to build `cross` from
master at the moment. See [the relevant PR](https://github.com/rust-embedded/cross/pull/344/files)

However, for the second example, as well as for automated CI systems, it's not
quite so easy. Then we need to set up more things, thus the example above. The
second example links dynamically to system libraries, which requires the linker
and build system to find headers and binaries for the right platform at compile
and link time.

To prepare debian for cross compilation, we do:

    ::shell
    dpkg --add-architecture armhf
    apt-get update
    apt-get install -y curl git build-essential
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf

First it sets up `armhf` (arm hard float, armv7 with hardware floating point),
then it updates package lists. The first installation is for basic build
systems (gcc, make, glibc-headers, etc) and the second is for the cross
compiler environment.

Then, for [example#2](https://gitlab.com/Spindel/rust-cross-example/-/blob/master/cross_example_02/src/main.rs)
we need to add dbus development headers (including pkg-config and more) for
both the platforms we have. 

After this, I install the rust system, in the most horrible way ever, by
running code directly from the internet. (shh. don't think too much about what
a container is)

    ::shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh
    bash /usr/local/bin/rustup.sh -y
    rustup default stable
    rustup target add x86_64-unknown-linux-gnu
    rustup target add armv7-unknown-linux-gnueabihf

Then we let rustup add the two compiler chains for rust, x86_64 and armv7.
Normally, it defaults to adding the native architecture, but I like my examples
to be overly explicit.

Then comes the two magic export lines:

    ::shell
    export PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
    export PKG_CONFIG_ALLOW_CROSS="true"

`PKG_CONFIG_PATH` is used by [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/)
to find it's development files. And pkg-config is a tool that lets libraries
(dbus, above) drop in snippets which tell build systems (cargo, in this case)
of how to find and link against the system libraries.

And the `PKG_CONFIG_ALLOW_CROSS` is the magic incantation to tell the crate
[pkg-config-rs](https://github.com/rust-lang/pkg-config-rs#external-configuration-via-target-scoped-environment-variables)
to permit cross compilation.

Once all this is done, you should be able to compile example 02 using:
   
    ::shell
    cargo build --target armv7-unknown-linux-gnueabihf

There's a complete
[Dockerfile](https://gitlab.com/Spindel/rust-cross-example/-/blob/master/Dockerfile)
example to use if you want to, as well as a 
[GitLab CI](https://gitlab.com/Spindel/rust-cross-example/-/blob/master/.gitlab-ci.yml)
example to automate the process.

Do note, that for production CI usage, I'd recommend you push the finished
cross-compile capable container to a registry, and reuse it, so you don't waste
time, CI minutes, and power waiting for computers to download and install
packages over and over again.

Happy hacking!
