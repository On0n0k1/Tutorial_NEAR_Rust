# Lesson 6 - Enums

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/)

This lesson is sub-divided into four sessions. As each session will be a crate of its own, we will take the time to learn about workspaces and how to use them.

Here's a quick overview of sessions:
 - [Session 1 - Declaring and using Enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_1_simple/).
 - [Session 2 - Implementing a Thermometer using Enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_2_thermometer/).
 - [Session 3 - Pragmatic and Efficient error handling.](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_3_game_score/).
 
---

## Workspaces

A workspace is, quite simply, a directory shared by a set of packages. At compile time, all packages share the same target directory and Cargo.lock file, with each package compiling to a crate in that shared target directory. 

Here are some features of workspaces: 
 - Local dependencies: We create a package for our project, and then add packages our project depends on. 
 - Project management: we can compile and execute testing of all related packages and/or projects in one place. 
 - Smart Contract orchestration: A project usually has multiple smart contracts that handle different concerns (responsibilites). A workspace can include Smart Contracts in a test crate for testing cross-contract calls. There's such a test tool called [workspaces-rs](https://github.com/near/workspaces-rs) which can help you quite a lot when developing your own Smart Contracts.
 

---

## Cargo.toml
[top](#workspaces)

You need to add a special section called `[workspace]`.

```toml
[workspace]
members = [
    "lesson_6_1_simple",
]
```

`members` is a list of packages (projects) managed in this workspace.

:hand: **NOTE:** every package found in the workspace directory, will be included in the workspace. You can also leave packages out, using the `exclude` key in the `[workspace]` section. [Learn more](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section) about it. 


```toml
[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]
```
In the example above, we included 3 paths, and excluded 2. There's support for wildcards, called [globs](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) (or Unix shell style patterns), to match multiple paths. 

---

## Using the CLI with workspaces
[top](#workspaces)

If we execute the command `cargo build` and `cargo test` in a package that is a member of a workspace, that command will apply to all other packages that are members of that workspace. 

If we want to limit the command's execution to a single package, we can add the flag `-p` / `--package` or `--workspace` to the command. 

Let's test `lesson_6_1_simple` by running the following command:

```bash
cargo test -p lesson_6_1_simple -- --nocapture --test-threads=1
```

`--nocapture` will print all output.

`--test-threads=1` will run all tests in one thread, making the output legible.

Now, let's compile our package to our wasm target and a (fully optimized) **release** version, by running: 

```bash
cargo build -p lesson_6_1_simple --target wasm32-unknown-unknown --release
```
WASM (WebAssembly) files will be located in './lesson_6_enums/target/wasm32-unknown-unknown/release/'.

Finally, let's generate our documentation. Let's run: 
```bash
cargo doc --lib --document-private-items -p lesson_6_1_simple --open
```
This will generate all documentation, and open it in the default browser. 

 - `--lib` specifies this is a library.
 - `--document-private-items` generates documentation for all items.
 - `--open` will open the default browser pointing to the generated documentation. 

All documentation will be located in './target/doc/lesson_6_1_simple/index.html'.


---

Lesson 6 - Intro :white_check_mark: ... **Done! Congratulations!**

Let's move on to the next section to learn more about [declaring and using Enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/lesson_6_1_simple/).
