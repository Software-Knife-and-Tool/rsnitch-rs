#
# rsnitch-rs makefile
#
.PHONY: help debug release commit clean

help:
	@echo "rsnitch-rs top-level makefile -----------------"
	@echo
	@echo "--- build options"
	@echo "    release - build release/optimized"
	@echo "    debug - build debug"
	@echo "--- development options"
	@echo "    commit - run tests, clippy, rustfmt"
	@echo "    clean - remove build artifacts"

release:
	@cargo build --quiet --release

debug:
	@cargo build --quiet

commit:
	@cargo fmt
	@echo ";;; rust tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy --quiet

clean:
	@rm -rf target
	@cargo update
