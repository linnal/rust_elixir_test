# Rustelixir

###NIFs - Native Implemented Functions

NIFs is s a feature available in Erlang that allows to call code implemented in C/C++/Rust from Elixir, the native code is compiled into a shared library.

Generally using NIFs comes with some warnings:
  - A crash in the native function will crash the all VM. => No fault tolerance anymore; 
  - A native function doing lengthy work degrades responsiveness of the VM and can cause strange behaviours (extreme memory usage; bad load balancing between schedulers). 

They are called as any other Elixir function and they replace the Elixir functions of the same name during their load time. 


**Why write a NIF?**

- [Using Rust to Scale Elixir for 11 Million Concurrent Users](https://blog.discordapp.com/using-rust-to-scale-elixir-for-11-million-concurrent-users-c6f19fc029d3)
- a functionality you need already exist in a native library

## Arguments

`fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>>`

- env is passed as a first argument to all NIFs. It represents an environment that can host Erlang terms and it contains information about the calling Erlang process. It allows you to decode/encode terms using this environment.

- Term refers to any Erlang term. They are used as arguments to API functions or as return values from NIFs. All terms in an environment are valid as long as the environment is valid

- args are the arguments you pass to the function


## Why Rust?
- Fast compiler
- Prevents Segmentation Fault
- Guarantees Thread safety

This solves the fault tolerance warning. 
[Rustler](https://github.com/rusterlium/rustler) is a library for writing Erlang NIFs in safe Rust code. That means there should be no ways to crash the BEAM (Erlang VM). The library provides facilities for generating the boilerplate for interacting with the BEAM, handles encoding and decoding of Erlang terms, and catches rust panics before they unwind into C. Provides mix command to generate a boilerplate. 

## NIF Resource Object
Return pointers to native data structure from a NIF. It can be stored and passed between processes of the same node and it is not deallocated until there is no more processes having references to the data structure. As an example you can create a Rust struct containing a binary buffer (or any thing else), put that in a resource object, and use that as an argument to subsequent NIF calls. This approach has many advantages, which includes very fast mutation and random access to the buffer, with no need to leave the current process.


## More
[Blog](http://hansihe.com/2017/02/05/rustler-safe-erlang-elixir-nifs-in-rust.html)