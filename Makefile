ARGS:=
WAVES:=

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

test: test_rust test_grep

test_rust:
	cargo test --no-fail-fast --all | tee test.log

test_grep:
	grep "test result" test.log


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


bridge_rc: waves/stamp log/stamp
	cargo test --no-fail-fast \
		--test test_trans_ir_bridge_rc_load \
		-- --nocapture | tee log/bridge_rc.log

trans: waves/stamp log/stamp
	cargo test --no-fail-fast \
		--test test_trans_ir_bridge_loaded \
		-- --nocapture | tee log/trans.log

trans_plot: trans
	kst2 kst/trans.kst


cap_dc:
	cargo test --no-fail-fast \
		--test test_irrc

cap_lpf: log/stamp waves/stamp
	cargo test --no-fail-fast \
		--test test_trans_irrc \
		-- --nocapture | tee log/trans_lpf.log

cap_hpf: log/stamp waves/stamp
	cargo test --no-fail-fast \
		--test test_trans_irrc_hpf \
		-- --nocapture | tee log/trans_hpf.log


waves/stamp:
	mkdir -p waves; touch waves/stamp

log/stamp:
	mkdir -p log; touch log/stamp

clean:
	cargo clean
	\rm -rf *.log
	\rm -rf log
	\rm -rf waves
