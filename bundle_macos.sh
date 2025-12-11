#!/bin/bash
set -e

APP_NAME="Toy Piano"
APP_DIR="$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "🚧 Bundling $APP_NAME.app..."

# 1. Clean previous build
rm -rf "$APP_DIR"

# 2. Create Directory Structure
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# 3. Copy Binary
echo "   Copying binary..."
cargo build --release
cp target/release/toy-piano "$MACOS_DIR/ToyPiano"

# 4. Copy Assets
echo "   Copying assets..."
cp -r assets "$MACOS_DIR/"

# 5. Create Info.plist
echo "   Creating Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>ToyPiano</string>
    <key>CFBundleIdentifier</key>
    <string>com.jergas.toypiano</string>
    <key>CFBundleName</key>
    <string>Toy Piano</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
</dict>
</plist>
EOF

# 6. Create ICNS Icon (using a temporary iconset)
echo "   Creating AppIcon.icns..."
ICON_SOURCE="assets/abstract-soundwave-icon.png"
ICONSET_DIR="ToyPiano.iconset"

mkdir -p "$ICONSET_DIR"
# Resize to standard icon sizes
sips -z 16 16     "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_16x16.png" > /dev/null
sips -z 32 32     "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_16x16@2x.png" > /dev/null
sips -z 32 32     "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_32x32.png" > /dev/null
sips -z 64 64     "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_32x32@2x.png" > /dev/null
sips -z 128 128   "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_128x128.png" > /dev/null
sips -z 256 256   "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_128x128@2x.png" > /dev/null
sips -z 256 256   "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_256x256.png" > /dev/null
sips -z 512 512   "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_256x256@2x.png" > /dev/null
sips -z 512 512   "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_512x512.png" > /dev/null
sips -z 1024 1024 "$ICON_SOURCE" --setProperty format png --out "$ICONSET_DIR/icon_512x512@2x.png" > /dev/null

# Make icns
iconutil -c icns "$ICONSET_DIR" -o "$RESOURCES_DIR/AppIcon.icns"
rm -rf "$ICONSET_DIR"

echo "✅ Done! $APP_NAME.app is visible in this directory."
echo "   Try running: open '$APP_NAME.app'"
