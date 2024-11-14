install:
	cargo build --release 
	sudo mv ./target/release/sqleibniz /usr/bin/sqleibniz

uninstall:
	sudo rm /usr/bin/sqleibniz

examples:
	@# disabling sqleibniz specific diagnostics via -D
	cargo run -- \
		--ignore-config \
		-Dno-statements \
		-Dno-content \
		-Dunimplemented \
		-Dbad-sqleibniz-instruction \
		$(shell find ./example -name "*.sql")

cov:
	cargo tarpaulin --out Html
	python3 -m http.server 8080
	xdg-open "http://localhost:8080/tarpaulin-report.html"

