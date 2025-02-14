#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f 2)

# Create builds directory
BUILD_DIR="builds"
mkdir -p "$BUILD_DIR"

echo -e "${BLUE}Building Rust24k v${VERSION}${NC}"
echo -e "${BLUE}================================${NC}"

# Function to build for a specific target
build_target() {
    local target=$1
    local binary_name=$2
    
    echo -e "${BLUE}Building for ${target}...${NC}"
    
    # Check if target is installed, if not install it
    if ! rustup target list | grep -q "${target} (installed)"; then
        echo -e "${GREEN}Installing target ${target}...${NC}"
        rustup target add "$target"
    fi
    
    # Build
    cargo build --release --target "$target"
    
    # Check if build was successful
    if [ $? -eq 0 ]; then
        # Create target directory
        mkdir -p "${BUILD_DIR}/${target}"
        
        # Copy binary and rename
        if [[ $binary_name == *.exe ]]; then
            cp "target/${target}/release/rust24k.exe" "${BUILD_DIR}/${target}/${binary_name}"
        else
            cp "target/${target}/release/rust24k" "${BUILD_DIR}/${target}/${binary_name}"
        fi
        
        echo -e "${GREEN}✓ Build successful for ${target}${NC}"
    else
        echo -e "${RED}✗ Build failed for ${target}${NC}"
    fi
}

# Build for each target platform
build_target "x86_64-apple-darwin" "rust24k"         # Intel Mac
build_target "aarch64-apple-darwin" "rust24k"        # M1/M2 Mac
build_target "x86_64-pc-windows-gnu" "rust24k.exe"   # Windows x64
build_target "x86_64-unknown-linux-gnu" "rust24k"    # Linux x64
build_target "aarch64-unknown-linux-gnu" "rust24k"   # Linux ARM64

# Create zip archives for each build
echo -e "${BLUE}Creating archives...${NC}"
cd "$BUILD_DIR" || exit

for dir in */; do
    dir=${dir%/}
    zip -r "rust24k_${VERSION}_${dir}.zip" "$dir"
    echo -e "${GREEN}✓ Created archive for ${dir}${NC}"
done

cd ..

echo -e "${BLUE}================================${NC}"
echo -e "${GREEN}Build process completed!${NC}"
echo -e "Builds are available in the ${BUILD_DIR} directory" 