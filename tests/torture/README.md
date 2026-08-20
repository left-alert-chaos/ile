# Torture test
This is a testing suite for Ile. Specifically, it tests the language itself and its included standard library. This *does not* test Rust interop, and the only success condition is running the entire suite without returning an `Error`. While other tests will look for refusal to parse or run bad code, this is purely a matter of "does it run".
