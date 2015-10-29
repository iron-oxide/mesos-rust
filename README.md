# mesos-rust

Rust language bindings for [Apache Mesos](http://mesos.apache.org).

## Status

_**Note:** these bindings are not production-ready.  
Rust hackers welcome, this is really fun so far!_

There is a functioning example scheduler in the repo: [src/bin/test_scheduler.rs](src/bin/test_scheduler.rs).

## Project Roadmap

- [X] Provide a trait for Rust scheduler implementations.
- [X] Wire up scheduler callbacks for the native (libmesos) scheduler driver.
- [X] Implement native scheduler driver calls.
- [ ] Provide a trait for Rust executor implementations.
- [ ] Wire up executor callbacks for the native (libmesos) executor driver.
- [ ] Implement native executor driver calls.
- [ ] Implement scheduler and executor drivers based on the new HTTP APIs.
- [ ] Experiment with higher-level API constructs to ease Rust framework writing.

## Native Dependencies

- `libmesos.{so, dylib}`
- `libprotobuf-lite.{so, dylib}`

## Building `mesos-rust`

This project depends on Rust version 1.3.

Using [cargo](http://crates.io):

```
$ cargo rustc --bin test_scheduler -- -l mesos -l protobuf-lite
```

Using `make`:

```
$ make
```

Take it for a test drive!

```
$ mesos-local --num_slaves=2 &
$ target/debug/test_scheduler
```
