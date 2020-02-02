#!/usr/bin/env bash


# Build project
cargo clean
cargo build --release --bin cleanup
wait

# Target file
path=./target/release/cleanup.exe
checksum_file="checksums.txt"
filename="$(echo $path | awk -F'/' '{print $NF}')"
dirname="$(dirname $path)"
destfile="$dirname/$checksum_file"

# Generate checksums
sha1="$(shasum -a 1 $path | awk '{print $1}')"
sha256="$(shasum -a 256 $path | awk '{print $1}')"
sha512="$(shasum -a 512 $path | awk '{print $1}')"
md5sum="$(md5sum $path | awk '{print $1}')"

# File size in bytes and a human-readable format
bytes="$(wc -c $path | awk '{print $1}')"
readable_bytes="$(numfmt --to=si $bytes)"

# Save checksums to file
content="
File:    $filename
Size:    $readable_bytes ($bytes bytes)
MD5:     $md5sum
SHA-1:   $sha1
SHA-256: $sha256
SHA-512: $sha512
"

echo "$content" >> "$destfile"
echo "$content"
echo "Saved to $destfile"