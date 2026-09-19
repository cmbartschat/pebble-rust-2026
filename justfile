check-all:
	cargo check --features aplite --target thumbv7m-none-eabi --target-dir target/aplite
	cargo check --features basalt --target thumbv7em-none-eabi --target-dir target/basalt
	cargo check --features chalk --target thumbv7em-none-eabi --target-dir target/chalk
	cargo check --features diorite --target thumbv7em-none-eabi --target-dir target/diorite
	cargo check --features emery --target thumbv8m.main-none-eabi --target-dir target/emery
	cargo check --features flint --target thumbv7em-none-eabi --target-dir target/flint
	cargo check --features gabbro --target thumbv8m.main-none-eabi --target-dir target/gabbro

test-all:
	cargo test --features aplite,embedded-allocator,malloc-allocator --target thumbv7m-none-eabi --target-dir target/aplite
	cargo test --features basalt,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/basalt
	cargo test --features chalk,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/chalk
	cargo test --features diorite,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/diorite
	cargo test --features emery,embedded-allocator,malloc-allocator --target thumbv8m.main-none-eabi --target-dir target/emery
	cargo test --features flint,embedded-allocator,malloc-allocator --target thumbv7em-none-eabi --target-dir target/flint
	cargo test --features gabbro,embedded-allocator,malloc-allocator --target thumbv8m.main-none-eabi --target-dir target/gabbro
