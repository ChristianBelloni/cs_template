all:
	cargo build -r
	cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 -o ./android build --release 
	cargo pod build

csharp: all
	cargo test c_sharp

install-dep:
	@echo "Adding windows dependencies"
	rustup target add x86_64-pc-windows-msvc
	@echo "Adding android dependencies"
	rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
	@echo "Adding apple dependencies"
	rustup target add x86_64-apple-darwin aarch64-apple-darwin x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim

	@echo "Installing android ndk"
	-choco install android-ndk
	@echo "Installing cargo-ndk"
	cargo install cargo-ndk

	@echo "Installing cargo-pod"
	cargo install cargo-cocoapods