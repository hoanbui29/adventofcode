run-rust:
	@echo "Running Rust program..."
	@time cargo run --manifest-path ./rust/Cargo.toml

run-go:
	@echo "Running Go program..."
	@time go run ./go/main.go

run-node:
	@echo "Running Node program..."
	@time npx tsx ./node/main.ts

run: run-rust run-go run-node
