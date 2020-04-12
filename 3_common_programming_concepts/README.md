# Common Programming Concepts

## Variables and mutability
Variables are immutable by default. If you need to change a variable it's possible to add the keyword `mut` to it.

**Difference between immutable and const** 

If you come from Javascript you might wonder if immutable is the same situation we have when we use `const` in Javascript.

The big difference is that const variables in Rust cannot be mutated like the immutable variables you can create.

The seconds one is that its value cannot be binded by the result of other operation. Its value must be binder to a constant expression.

## Shadowing
If you declare two variables with the sme name in the same scope the first one will be shadowed by the second one.

This comes handy when you need to transform the value of some variable, but you don't want to think about another variable name
to contain the new value.

You can even change the type of the new variable and keep the same name.

## Data Types

Rust has scalar and compound data types. Rust is compiled so it is necessary to know the type of variables during compile time.

### Scalar types

Scalar types are types that represent a single value. Rust has four scalar types:

#### Integer

#### Floating-point numbers

#### Booleans

#### Char

### Compound Types

#### Tuple

#### Array


## Functions

## Ownership
Rust has an important feature called ownership. That basically means that any variable has an owner.

Whenever the parent scope is finished its variables will be cleaned out of memory. This way Rust doesn't need to have a garbage collector to keep checking what has to be cleaned. It brings speed.

All values implement the Drop trait, that means after that value goes out of scope. Rust will clean the memory that was allocated so you don't need to do that yourself.

Values can be stored in stack or in the heap. The stack has a FILO system.

All the values that the compiler knows its size at compile time are stored in the stack (int, float, arrays, tuples with same type, chars, string literals) that means if you pass this value to another owner the value will be copied as it's cheap to copy values from the stack.

These values implement the Copy trait.

For more complicated values which we can't know their size at compile time (vectors, String), Rust allocates a reference to that values in the stack (a pointer) and then stores the values in the heap.

These values implement the Move trait. That means if you pass the variable to another scope then that scope will now own your variable and the first reference will be moved to the new one. The first variable will not be available anymore.

Rust does that because if you keep references in different places they can try to clean the same memory space at the same time and as copying values in the stack is not cheap Rust doesn't copy them like the values that implement the Drop trait.

## Borrowing

It's possible to pass a reference to some value to another score by using `&`.
This way the owner, the reference in memory, will not be moved.
The new scope can now refer to a value, but doesn't own it. When its scope ends Rust will not drop anything.

Passing references is what is called Borrowing.










