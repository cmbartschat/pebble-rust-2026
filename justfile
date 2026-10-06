check-all:
	cargo check --features aplite --target thumbv7m-none-eabi --target-dir target/aplite
	cargo check --features basalt --target thumbv7em-none-eabi --target-dir target/basalt
	cargo check --features chalk --target thumbv7em-none-eabi --target-dir target/chalk
	cargo check --features diorite --target thumbv7em-none-eabi --target-dir target/diorite
	cargo check --features emery --target thumbv8m.main-none-eabi --target-dir target/emery
	cargo check --features flint --target thumbv7em-none-eabi --target-dir target/flint
	cargo check --features gabbro --target thumbv8m.main-none-eabi --target-dir target/gabbro
	(cd examples/multiple-touch-handlers && cargo check)
	(cd examples/random && cargo check)
	cargo clippy --features aplite --target thumbv7m-none-eabi
	cargo clippy --features basalt --target thumbv7em-none-eabi
	cargo clippy --features chalk --target thumbv7em-none-eabi
	cargo clippy --features diorite --target thumbv7em-none-eabi 
	cargo clippy --features emery --target thumbv8m.main-none-eabi
	cargo clippy --features flint --target thumbv7em-none-eabi
	cargo clippy --features gabbro --target thumbv8m.main-none-eabi
	(cd examples/multiple-touch-handlers && cargo clippy)
	(cd examples/random && cargo clippy)

fix-all:
	cargo clippy --fix --allow-dirty --allow-staged -q --all-targets
	cargo fix --allow-dirty --allow-staged -q --all-targets
	cargo fmt
	(cd examples/multiple-touch-handlers && cargo clippy --fix --allow-dirty --allow-staged -q --all-targets)
	(cd examples/multiple-touch-handlers && cargo fix --allow-dirty --allow-staged -q --all-targets)
	(cd examples/multiple-touch-handlers && cargo fmt)
	(cd examples/random && cargo clippy --fix --allow-dirty --allow-staged -q --all-targets)
	(cd examples/random && cargo fix --allow-dirty --allow-staged -q --all-targets)
	(cd examples/random && cargo fmt)

test-all:
	cargo test --features aplite,embedded-allocator,malloc-allocator --target thumbv7m-none-eabi --target-dir target/aplite
	cargo test --features basalt,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/basalt
	cargo test --features chalk,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/chalk
	cargo test --features diorite,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/diorite
	cargo test --features emery,embedded-allocator,malloc-allocator --target thumbv8m.main-none-eabi --target-dir target/emery
	cargo test --features flint,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/flint
	cargo test --features gabbro,embedded-allocator,malloc-allocator --target thumbv8m.main-none-eabi --target-dir target/gabbro

# Allows you to check that the logging infrastructure does not contain expensive calls to memcpy.
no-memcpy-in-logging:
	#!/usr/bin/env bash
	cd examples/logging
	cargo pebble build
	if arm-none-eabi-nm -CSn --size-sort target/pebble/build/aplite/logging.elf | grep memcpy; then
		echo 'memcpy found in the logging example!'
		exit 1
	fi
