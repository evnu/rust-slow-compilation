# Rust Slow Compilation

Reproducing https://github.com/rusterlium/rustler/issues/299 with a simpler example.

## Run the Benchmark

```
$ ./benchmark.sh
```

This will output a markdown table similar to the following:

|commit|measurement|desc|
|------|-----------|----|
|d90890b|117.12|Initial slow example|
|03155fa|45.16|Impl on Struct instead of trait impl|
|3dd603f|49.97|Use `use` to make it look simpler|
|d650ffc|59.90|Remove ::rustler::Error|
|fd04a12|56.11|Remove Box|
|0f4eaf3|0.91|Replace match with map_err|
|dc7dd61|52.25|Remove &str from Err|
|c1dfe4f|57.43|Remove usize|
|0c09343|0.49|Try to replicate with a local crate for Decoder|

## Adding a new benchmark

`benchmark.sh` search for commits with a subject line prefixed with `bench:`.
To add another benchmark, simply add a new commit with that prefix.
