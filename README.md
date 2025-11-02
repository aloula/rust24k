# 🎨 Rust24k - Image to UHD Converter

A high-performance Rust utility that converts images to UHD (3840x2160) format while preserving aspect ratio and EXIF orientation. Images are centered on a black background and named using their original timestamp.

## ✨ Features

- 🖼️ **UHD Conversion**: Converts images to 3840x2160 resolution
- 📐 **Aspect Ratio Preservation**: Maintains original proportions
- 🎯 **Smart Centering**: Centers images on black background
- 🧭 **EXIF Orientation**: Preserves and applies EXIF orientation data
- 📅 **Timestamp Naming**: Uses original photo timestamp for filenames
- 🔄 **Batch Processing**: Processes all JPG/JPEG files in directory
- 🚀 **High Performance**: Multi-threaded processing with progress indicators
- 🌐 **Cross-platform**: Available for multiple operating systems and architectures

## 🏗️ Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| 🍎 **macOS** | Intel (x86_64) | ✅ |
| 🍎 **macOS** | Apple Silicon (ARM64) | ✅ |
| 🪟 **Windows** | x86_64 | ✅ |
| 🪟 **Windows** | ARM64 | ✅ |
| 🐧 **Linux** | x86_64 | ✅ |
| 🐧 **Linux** | ARM64 | ✅ |

## 🚀 Installation

### Download Pre-built Binaries

1. Go to the [Releases](../../releases) page
2. Download the appropriate binary for your platform
3. Extract the archive
4. Place the executable in your PATH or use directly

### Build from Source

#### Prerequisites
- Rust 1.70+ installed ([Install Rust](https://rustup.rs/))
- Git

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/aloula/rust24k.git
cd rust24k

# Build optimized release
cargo build --release

# The binary will be available at: target/release/rust24k
```

#### Cross-compilation
Use the provided build script to compile for all supported platforms:

```bash
chmod +x build.sh
./build.sh
```

## 📖 Usage

### Basic Usage
```bash
# Navigate to directory containing images
cd /path/to/your/images

# Run the converter
rust24k
```

### Advanced Usage
The tool automatically:
- Scans the current directory for `.jpg` and `.jpeg` files
- Creates a `converted/` subdirectory for output
- Shows a progress bar during processing
- Preserves original files (non-destructive)

### Output Format
- **Resolution**: 3840x2160 (UHD/4K)
- **Format**: JPEG with optimized quality
- **Background**: Solid black (#000000)
- **Filename**: Based on EXIF timestamp or file modification time

## 🔧 Configuration

Currently, Rust24k uses sensible defaults. Future versions may include:
- Custom output resolution
- Background color options
- Quality settings
- Output format selection

## 📁 Project Structure

```
rust24k/
├── src/
│   └── main.rs          # Main application logic
├── builds/              # Cross-compiled binaries
├── converted/           # Output directory (created automatically)
├── Cargo.toml           # Rust dependencies and metadata
├── build.sh            # Cross-platform build script
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## 🛠️ Dependencies

- **[image](https://crates.io/crates/image)** - Image processing and format support
- **[walkdir](https://crates.io/crates/walkdir)** - Directory traversal
- **[kamadak-exif](https://crates.io/crates/kamadak-exif)** - EXIF metadata parsing
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling
- **[indicatif](https://crates.io/crates/indicatif)** - Progress bars and indicators

---

**Made with ❤️ in Rust**

