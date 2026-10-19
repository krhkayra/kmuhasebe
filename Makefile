all:
	QMAKE=/usr/bin/qmake6 cargo build --release
clean:
	rm -rf Cargo.lock
	rm -rf target

