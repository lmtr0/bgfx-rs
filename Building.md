# Compiling
if this is the first time, be aware that it will download and compile the bgfx library, which may take a while.
```sh
cargo build
```

## Cross compiling
> ATTENTION!!!
cross compilation is currently only supported on linux. <br>
if you are compiling for macos, you will need to compile the [osxcross](https://github.com/tpoechtrager/osxcross) with OSX_VERSION_MIN=10.7 setted and have a SDK greated than 10.12 <br>
if you are compiling for windows mingw, you will need to install mingw