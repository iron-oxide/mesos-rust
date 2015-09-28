all: test-scheduler

clean:
	cargo clean

test:
	cargo test

test-scheduler:
	cargo rustc --bin test_scheduler -- -l mesos -l protobuf-lite
