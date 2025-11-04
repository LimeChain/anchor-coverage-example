PLATFORM_TOOLS_VERSION=v1.52

all:

coverage_stats:
	cargo-build-sbf --tools-version ${PLATFORM_TOOLS_VERSION} --debug --arch v1 ; rm -rf sbf_trace_dir ; SBF_TRACE_DIR=sbf_trace_dir cargo test -- --nocapture ; RUST_BACKTRACE=1 SBPF_VERSION=v1 TOOLS_VERSION=${PLATFORM_TOOLS_VERSION} SBF_TRACE_DIR=sbf_trace_dir ./anchor-coverage-dwarf/target/debug/anchor-coverage
	genhtml --output-directory coverage sbf_trace_dir/*.lcov --rc branch_coverage=1 && open coverage/index.html
