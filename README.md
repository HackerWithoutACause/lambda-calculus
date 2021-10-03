# λ (Lambda) Calculus Solver

This is a [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus)
solver/reducer I wrote to better understand what it is and how it works. I
choose Rust as my language of choose because it is the one I feel the most
comfortable programming in.

## Quick Start

In order to compile and run `λ-calculus` you need Rust and Cargo installed, then
simply do:

```
git clone https://github.com/HackerWithoutACause/lambda-calculus
cd lambda-calculus
cargo run --release -- 'your expression'
```

## Future Work

* Support passing a file as input rather than just though command line arguments.
* Constant definitions for standard terms and others.
* Multi-character variables.
* Support for [typed lambda calculus](https://en.wikipedia.org/wiki/Typed_lambda_calculus).
