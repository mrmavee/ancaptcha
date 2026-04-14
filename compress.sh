#!/usr/bin/env bash
# This script converts original images (JPG, JPEG, PNG) to 200x200 WebP for anCaptcha.
# Requires: ffmpeg with libwebp support.

set -euo pipefail

shopt -s nocaseglob nullglob

if ! command -v ffmpeg &> /dev/null; then
    echo "Error: ffmpeg is not installed. Please install it to continue."
    exit 1
fi

mkdir -p crates/ancaptcha/assets/default_images

img_files=(crates/ancaptcha/assets/original_images/*.{jpg,jpeg,png})

if [[ ${#img_files[@]} -eq 0 ]]; then
    echo "No images found in original_images directory."
    exit 0
fi

for img_path in "${img_files[@]}"; do
    [[ -e "$img_path" ]] || continue
    
    img_name=$(basename "$img_path")
    img_stem="${img_name%.*}"
    
    echo -n "Processing: $img_stem... "
    
    met_limit=false
    for quality in {40..1..-5}; do
        ffmpeg -i "$img_path" -vf scale=200:200:flags=lanczos -c:v libwebp -q:v "$quality" -compression_level 6 -y "crates/ancaptcha/assets/default_images/$img_stem.webp" > /dev/null 2>&1
        
        current_size=$(stat -c%s "crates/ancaptcha/assets/default_images/$img_stem.webp")
        
        if [[ $current_size -le 3500 ]]; then
            echo "Done (Quality: $quality, Size: $current_size bytes)"
            met_limit=true
            break
        fi
    done

    if [[ "$met_limit" == "false" ]]; then
        echo -e "\n[WARNING] $img_stem.webp exceeded size limit (3500 bytes). Current size: $current_size bytes at quality 1."
    fi
done

echo "Processing complete."
