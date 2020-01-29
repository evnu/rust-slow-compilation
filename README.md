# Rust Slow Compilation

Reproducing https://github.com/rusterlium/rustler/issues/299 with a simpler example.

## Run the Benchmark

```
$ ./benchmark.sh
```

This will output a markdown table similar to the following:

|commit|measurement|desc|
|------|-----------|----|
|d90890b|50.21|Initial slow example|
|03155fa|46.47|Impl on Struct instead of trait impl|
|3dd603f|51.42|Use `use` to make it look simpler|
|d650ffc|56.39|Remove ::rustler::Error|
|fd04a12|55.17|Remove Box|
|0f4eaf3|0.87|Replace match with map_err|

## Adding a new benchmark

`benchmark.sh` search for commits with a subject line prefixed with `bench:`.
To add another benchmark, simply add a new commit with that prefix.
