#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status.

echo "Starting iOS build process..."

# Set iOS deployment target
export IPHONEOS_DEPLOYMENT_TARGET=13.4

# Cargo Build
cargo build && cd pubky && cargo build && cd pubky && cargo build && cd ../ && cd pubky-common && cargo build && cd ../ && cd pubky-homeserver && cargo build && cd ../../

# Modify Cargo.toml
echo "Updating Cargo.toml..."
sed -i '' 's/crate_type = .*/crate_type = ["cdylib", "staticlib"]/' Cargo.toml

# Build release
echo "Building release version..."
cargo build --release

# Add iOS targets
echo "Adding iOS targets..."
rustup target add aarch64-apple-ios-sim aarch64-apple-ios

# Build for iOS simulator and device
echo "Building for iOS targets..."
cargo build --release --target=aarch64-apple-ios-sim
cargo build --release --target=aarch64-apple-ios

# Generate Swift bindings
echo "Generating Swift bindings..."
cargo run --bin uniffi-bindgen generate --library ./target/release/libpubkysocialmobile.dylib --language swift --out-dir ./bindings/ios


# Rename modulemap file
# mv bindings/pubkymobileFFI.modulemap bindings/module.modulemap

# Create XCFramework
echo "Creating XCFramework..."
xcodebuild -create-xcframework \
  -library ./target/aarch64-apple-ios-sim/release/libpubkysocialmobile.a -headers ./bindings \
  -library ./target/aarch64-apple-ios/release/libpubkysocialmobile.a -headers ./bindings \
  -output "ios/PubkyMobile.xcframework"

# Remove ios directory
echo "Removing ios directory..."
rm -rf ios/

echo "iOS build process completed successfully!"