Quadratic residue 

Quadratic residue is `x` which has a square root, i.e there exists `y`
such that `y^2` is congruent to `x mod p`


The circuit is written to prove `y = f(x)` for  a function `f: Fp -> u64`,
where the ith of `f(x)` is 1 if `x + i` is a quadratic residue and 0 otherwise for `i = [0..63]`

Run benchmarks
```
DEGREE=10 cargo test --package benchmarking --lib -- residue_pattern_benches::tests --show-output
```