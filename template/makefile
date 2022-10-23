all:
	cargo build -r
	cargo ndk -t armeabi-v7a -t arm64-v8a -o ./android build --release 

csharp:
	cargo build -r
	cargo ndk -t armeabi-v7a -t arm64-v8a -o ./android build --release 
	cargo test c_sharp
