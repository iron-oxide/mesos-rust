# mesos-rust

Rust language bindings for [Apache Mesos](http://mesos.apache.org).

_Note: this project is not ready to use for anything._

## Native Dependencies

- `libmesos.{so, dylib}`
- `libprotobuf-lite.{so, dylib}`

## Building `mesos-rust`

Using [cargo](http://crates.io):

```
$ cargo rustc --bin test_scheduler -- -l mesos -l protobuf-lite
```

Take it for a test drive!

```
$ mesos-local &
$ target/debug/test_scheduler
```

