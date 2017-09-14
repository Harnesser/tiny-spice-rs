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
	cargo test --no-fail-fast --all

newton:
	cargo test --no-fail-fast --test newton

diode:
	cargo test --no-fail-fast \
		--test test_ird \
		--test test_ir_drev \
		--test test_v_d_vs_d \
		--test test_irdvv \
		--test test_dc_bridge_p_unloaded \
		--test test_dc_bridge_p_loaded

trans:
	cargo test --no-fail-fast \
		--test test_trans_ir \
		-- --nocapture | tee trans.log

clean:
	cargo clean
	\rm -rf *.log
