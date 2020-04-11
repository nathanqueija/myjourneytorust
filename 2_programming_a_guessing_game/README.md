# Programming a guessing game

## Variables

Variables in Rust are immutable by default.

They are declared like this: `let a = 3;`

If you want to mutate its value you can use the keyword `mut`: `let mut a = 3;`

**Why let?**

I was curious about the meaning of let since we find the same usage in Javascript.

I made some research and found this explanation by [lookmeat](https://www.reddit.com/user/lookmeat/)  on Reddit:

    A synonym for let would be allow. In mathematics let is generally used to mean "make" but in this case it's because in math anything can be anything, but we want to focus on a specific case. Basically a variable can by any value, but we "let" it be a specific value to prove a point on that specific value.
    
    So let x = "hello"; is the same as saying "let/allow/make the variable x be equal to "hello"


## Packages aka Crates
If you come from Javascript and is used to work with NPM cargo will work similarly.

Cargo is also a depdency manager for Rust projects. You can add a new dependency just listing it on `Cargo.toml` file.

There is no command such as `npm install`. Cargo you install and compile the dependencies whenever you compile you application.

[Crates.io](https://crates.io/) is the registry where the packages live. Anyone can create and publish a crate there the same way Javascript developers can publish packages to NPM.

## Dependency versions

When you build your project for the first time cargo will create a file called `Cargo.lock`
that will keep track of all your dependency versions.
If any library updates a version of a library internally that you are also using in your project you won't be affected.
This is super nice as you always get a reproducible build and don't need to worry about versions out of sync.

Do you remember when you get error in your Typescript project because you have two different versions of the same package in your project because some library
inside node_modules installed a different version? That will not happen with Cargo. 💙
Your packages will be upgraded only if you explicitly do that.

Cargo also has the `update` command that will update a specific crate. It will try to figure out what's the best minor version to update.

If you want to update the crate to a major version you need to explicitly change the version number on `Cargo.toml`.

## How do I know how to use the library?

One of the greatest feature of cargo is that it builds the documentation for you.

If you run `cargo doc --open` it will not only build the documentation of your code, but also for all the dependencies you have listed.






