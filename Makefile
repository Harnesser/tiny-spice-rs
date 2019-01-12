ARGS:=
WAVES:=
TC:=ngspice/test_bridge_loaded_sine.spi

run:
	cargo run ${ARGS} ${TC} | tee sim.log

release: ARGS+=--release
release: run
debug:
	env RUST_BACKTRACE=1 cargo run

clippy:
	rustup run nightly cargo clippy

spice_t: 
	rm -rf ./spice; rustc --test src/spice.rs; ./spice --nocapture

test: test_rust test_grep

test_rust: waves/stamp log/stamp
	cargo test -j2 --no-fail-fast --all > log/test.log || echo 0

test_grep:
	grep "\.\.\." log/test.log | sort > doc/test_summary.txt; \
	cat doc/test_summary.txt


newton:
	cargo test --no-fail-fast --test newton

diode:
	cargo test \
		--test test_ird \
		--test test_ir_drev \
		--test test_v_d_vs_d \
		--test test_irdvv \
		--test test_dc_bridge_p_unloaded \
		--test test_dc_bridge_p_loaded

diode_lin: diode_lin_run
	grep DATA log/diode.log | sed -e "s/DATA //" > waves/diode_linearisation.dat

diode_lin_run:
	cargo test curve_trace -- --nocapture > log/diode.log

sweep_vrd:
	cargo test --no-fail-fast \
		--test test_sweep_v_rd \
		-- --nocapture | tee log/sweep_v_rd.log


bridge_rc: waves/stamp log/stamp
	cargo test --no-fail-fast \
		--test test_trans_ir_bridge_rc_load \
		-- --nocapture | tee log/bridge_rc.log

trans: waves/stamp log/stamp
	cargo test --no-fail-fast \
		--test test_trans_ir_bridge_loaded \
		-- --nocapture > log/trans.log

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

blah:
	egrep \
		--color=always \
		-i "Time|METRIC|shifting|<" \
		log/trans.log \
		| aha > log/interesting.html


# Run the _loop tests 

loop_halfbridge:
	sh bin/loop.sh test_trans_irrd_sine loop
loop_halfbridge_plot:
	python3 bin/r8n waves/test_trans_irrd_sine_loop -expr "3"

loop_fullbridge:
	sh bin/loop.sh test_trans_ir_bridge_loaded loop
loop_fullbridge_plot:
	python3 bin/r8n waves/test_trans_ir_bridge_loaded_loop -expr "4-5"


loop_rcbridge:
	sh bin/loop.sh test_trans_ir_bridge_rc_load loop
loop_rcbridge_plot:
	python3 bin/r8n waves/test_trans_ir_bridge_rc_load_loop -expr "4-5"

loop_lpf:
	sh bin/loop.sh test_trans_irrc lpf_loop
loop_lpf_plot:
	python3 bin/r8n waves/test_trans_irrc_lpf_loop -expr "3"


waves/stamp:
	mkdir -p waves; touch waves/stamp

log/stamp:
	mkdir -p log; touch log/stamp

dot:
	dot doc/value.dot -Tsvg > doc/value.svg

clean:
	cargo clean
	\rm -rf *.log
	\rm -rf log
	\rm -rf waves
