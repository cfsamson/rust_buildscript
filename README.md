# Example of using Rust for build tasks

I found myself copying a lot of files, needing to remember to include files and assets manually for small tools and programs. Since I develop on Mac most of the time but build and deploy on windows, using shell scripts did not seem ideal.

So instead, inspired by [this blog post](link1) i created this template project to use Rust to do all the tasks.

## Why I made this
Mainly for fun. But it works so i made this to use as a template and for remembering the steps to implement this in the future.

NB! Not tested on windows yet, I just wrote the commands as I remembered them

## What we do:

1. Create a project as normal "myproj"
2. run `cargo new tools` to create a tools project (this can be called whatever we want, it's just a normal Cargo binary project)
3. Remove `main.rs` from the new project
4. Edit the new `cargo.toml` and add `publish=false` to the `package` section of the config.
5. Create a bin folder the new project like this: `./myproj/tools/src/bin`
6. Create one or more binaries, my suggestion is one called `gen` that does all the build steps, but you can add as many as you want for different tasks.
7. Add the following to the parent project cargo.toml
    ```toml
    # in ./myproj/cargo.toml
    [workspace]
    members = ["tools"]
    ```
8. Create a folder called `.cargo`
9. Create a file in `.cargo` called `config` (note no extension on the file) and add:
    ```toml
    # in ./myproj/.cargo/config
    [alias]
    gen  = "run --package tools --bin gen"
    otherbinary = "run --package tools --bin otherbinary"
    ```

## Usage
Run `cargo gen` to run the example and build the project. It will do the actions from the program file in `./myproj/tools/src/bin/gen.rs` which is:
1. Creating an output folder (deleting it if it exists)
2. Building the rust project in --release mode
3. Copying the resulting binary to the output folder
4. Copying the configuration file to the output folder

The program is just ment as a short example to remember/copy the steps from.




[link1]:(https://matklad.github.io/2018/01/03/make-your-own-make.html)