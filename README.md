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

## Testing

When creating this evaluator I used several testing cases to prove my
implementation along with running troughs expression though other lambda
calculus evaluators specifically [lambdacalc.io](https://lambdacalc.io/) and
[one by Robert Kovacsics](https://www.cl.cam.ac.uk/~rmk35/lambda_calculus/lambda_calculus.html).

I used these example found in a
[code golf post](https://codegolf.stackexchange.com/questions/284/write-an-interpreter-for-the-untyped-lambda-calculus).
Of which I believe the last output to be incorrect as every other interpreter I
have running says the output is `λ b . λ d . b (b (b (b (b (b (b (b d)))))))`.

I also used a example found is the comments of the Stack Overflow post that test
for correct α conversion: `((λ f. (λ x. (f x))) (λ y. (λ x. y)))` becomes
`(λ x. (λ x'. x))` not `(λ x. (λ x. x))`

## Future Work

* Support passing a file as input rather than just though command line arguments.
* Constant definitions for standard terms and others.
* Multi-character variables.
* Support for [typed lambda calculus](https://en.wikipedia.org/wiki/Typed_lambda_calculus).
