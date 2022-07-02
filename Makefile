default:
	cargo build
	cp "bin/debug/ferry" "bin/ferry"
	export PATH="/media/kittycat/Linux_files/rust/Ferry/bin:${PATH}"