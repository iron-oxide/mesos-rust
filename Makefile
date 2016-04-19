MESOS_INCLUDE_DIR="/usr/local/include/mesos"
MESOS_PROTO=$(MESOS_INCLUDE_DIR)/mesos.proto

all: test-scheduler

clean:
	cargo clean

test:
	cargo test

proto:
	@ echo "Checking for protoc"
	@ if ! which protoc > /dev/null; then \
	echo "Error: protoc not installed" >&2; \
	exit 1; \
	fi
	@ if ! protoc --version | grep 'libprotoc 2\.5\.' > /dev/null; then \
	echo "Error: this project requires protobuf 2.5" >&2; \
	exit 1; \
	fi
	@ echo "Checking for protoc-gen-rust"
	@ if ! which protoc-gen-rust > /dev/null; then \
	echo "Error: protoc-gen-rust not installed" >&2; \
	echo "Install with Cargo: 'cargo install protobuf'" >&2; \
	exit 1; \
	fi
	@ echo "Checking whether the Mesos include directory is sane"
	@ if ! stat $(MESOS_PROTO) > /dev/null; then \
	echo "Error: could not locate the Mesos protobuf definitions in '$(MESOS_INCLUDE_DIR)'" >&2; \
	echo "You may need to override MESOS_INCLUDE_DIR and run make with '-e'" >&2; \
	exit 1; \
	fi
	@ echo "Generating protobuf bindings"
	protoc --rust_out=src/proto --proto_path=/usr/local/include/mesos $(MESOS_PROTO)

test-scheduler:
	cargo rustc --bin test_scheduler -- -l mesos -l protobuf-lite
