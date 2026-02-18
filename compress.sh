#!/bin/bash
# This script converts original images (JPG, JPEG, PNG) to 200x200 WebP for anCaptcha.
# Requires: ffmpeg with libwebp support.

mkdir -p crates/ancaptcha/assets/default_images

for f in crates/ancaptcha/assets/original_images/*.{jpg,jpeg,png}; do
    [ -e "$f" ] || continue
    filename=$(basename "$f")
    filename="${filename%.*}"
    for q in {40..1..-5}; do
        ffmpeg -i "$f" -vf scale=200:200:flags=lanczos -c:v libwebp -q:v $q -compression_level 6 -y "crates/ancaptcha/assets/default_images/$filename.webp" > /dev/null 2>&1
        size=$(stat -c%s "crates/ancaptcha/assets/default_images/$filename.webp")
        if [ $size -le 3500 ]; then
            break
        fi
    done
done
