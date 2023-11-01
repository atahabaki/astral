alias cr := commit_ready
alias gd := gen_doc
alias rr := run_release
alias rd := run

run:
	cargo run

run_release:
	cargo run --relase

gen_doc:
	cargo doc --open

commit_ready:
	cargo fmt
	cargo test
	cargo clippy
	cargo check