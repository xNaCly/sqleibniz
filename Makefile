install:
	cargo build --release 
	sudo mv ./target/release/sqleibniz /usr/bin/sqleibniz

uninstall:
	sudo rm /usr/bin/sqleibniz

