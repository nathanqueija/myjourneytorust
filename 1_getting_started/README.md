# Getting Started

## 1. Installation
[Rustup](https://github.com/rust-lang/rustup) is a toolchain that installs and manages Rust for you.

It's super easy to work with. When you install Rust using it you'll get `rustc`, `cargo`, etc. It's super nice to have 
one tool that deals with all the dependencies you need to start coding. It took me literally 30 seconds to start coding.
Just one command, and we're set up. No errors. No pain.

### Rustc
`rustc` is the Rust's compiler. Rust is a compiled language.
That means you need to compile all your code and translate it into machine language (0s and 1s) before running it.
During this time the compiler will check for any flaws in your code and won't let you compile if there is something wrong.

It might be a bit frustrating in the beginning because the compiler is very strict, but I can already see the benefits
of not trusting yourself to cover all the edge cases of your code.
The compiler is strict because we need help to code better 😄

One nice thing about the compiler is that sometimes it won't compile my code because of a lot of possible errors that 
could happen, and the outcome is that I'll need to rethink how I designed my application. Even though the compiler does not
enforce any design guide it often helps you to reach better design by refactoring.
Do you know when you feel that things are not in the place they should be? The compiler
won't tell you that, but by fixing the errors you'll realize some of this improvements.
It's great.

You can compile any rust code by running `rustc {path for the file}` and if everything goes well
you should have a binary executable next to your file.

You can run ```rustc hello_world.rs``` to see this in action.

This command is fine for small programs, but as your projects grow there are better ways to manage
this process using `cargo`.

#### How is this process different from coding with Javascript?

As you saw compiling and running your code with Rust are two separate steps.

It means that Rust is an `ahead-of-time or AOT` compiled language. Javascript is a `just-in-time or JIT` language (since 2008 when V8 appeared). It's interpreted.

The interpreter does its job during runtime. The compiler does its job before runtime.


With Rust after you compiled your code a binary executable will be generated, and you can give this executable for anyone
to execute it without having Rust installed because Rust was compiled and translated to machine code in the compilation phase.

With Javascript the person who's executing your code needs to have a Javascript interpreter/implementation installed, for instance,
the browser, Node, etc.

If you want to know more about AOT and JIT compilers I can recommend:

[Essentials of Interpretation. Lecture [3/18] Compilers: AOT, JIT, Transpiler](https://www.youtube.com/watch?v=r1S9N4if__A)

[A crash course in just-in-time (JIT) compilers](https://hacks.mozilla.org/2017/02/a-crash-course-in-just-in-time-jit-compilers/)

[Computer Programming for Beginners | What are Interpreters, Compilers & JIT compilers? | Ep18](https://www.youtube.com/watch?v=svJerixawV0)

### Cargo

Think of cargo as NPM + Webpack (or any other similar tool) + Babel + Eslint + Prettier in just one tool. And you don't even need to provide any configuration files.

Isn't it a dream? Cargo is Rust's build system and dependency manager.

It doesn't mean that all these tasks are done by cargo, but it provides commands that will at least redirect the action to the responsible source.

When you install rust with rustup it also installs Cargo so it's not needed to install anything else.

If you create a new project using `cargo new` you'll see a `Cargo.toml` which corresponds to what `package.json` is in a Node project.

Packages in Rust are referred to as crates. So if you say you want to install an NPM package in a Node project you'd say you want to install a create in a Rust project.










 
