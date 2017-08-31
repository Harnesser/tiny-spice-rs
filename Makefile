ARGS:=

go:
	cargo run bringup

run:
	cargo run ${ARGS} | tee sim.log

release: ARGS+=--release
release: run
debug:
	env RUST_BACKTRACE=1 cargo run

clippy:
	rustup run nightly cargo clippy


waves:
	gtkwave --dump waves.vcd --save plot.gtkw

test:
	cargo test --no-fail-fast

newton:
	cargo test --no-fail-fast --test newton

clean:
	cargo clean
	\rm -rf *.log
