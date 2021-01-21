build:
	command -v x86_64-linux-musl-gcc > /dev/null 2>&1 || brew install filosottile/musl-cross/musl-cross
	CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/handler .

deploy-infra:
	cd infrastructure && \
	bicep build main.bicep && \
	az deployment sub create --location australiaeast --template-file main.json

deploy-app:
	func azure functionapp publish rustfunction

deploy: deploy-infra deploy-app