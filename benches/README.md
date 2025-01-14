# Benchmarking

Benchmarking numbers will vary across tests, specific grammars, rust versions, and hardware. With so many sources of noise, it is important to remember that "faster" is not always easy to define.

With that in mind, BNF's benchmarking has the following goals:
* identify statistically significant performance regressions
* validate performance hypothesis

These benchmarks are not run during continuous integration testing. But if a developer is attempting a full overhaul of the BNF parser, these benchmarks will help them identify large changes in performance.

## How to Run

> cargo bench

## Criterion

[Criterion][criterion] is used to run benchmarks with confidence. Criterion achieves this by running warmups, collecting a statistically significant sized sample, and generating informative reports.

## Cargo Criterion

[cargo-criterion][cargo-criterion] is a plugin for Cargo which handles much of the heavy lifting for analyzing and reporting on Criterion-rs benchmarks. It eases generating performance report files.

### Install

> cargo install cargo-criterion

### How To Run

> cargo criterion

[criterion]: https://crates.io/crates/criterion
[cargo-criterion]: https://github.com/bheisler/cargo-criterion