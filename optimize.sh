#!/bin/sh

TARGET_DIR="."

find "$TARGET_DIR" -type f ! -iname "*.webp" -print0 | while IFS= read -r -d '' file; do
    dir=$(dirname "$file")
    base=$(basename "$file")
    filename="${base%.*}"
    
    output="${dir}/${filename}.webp"
    
    echo "Converting '$file' -> '$output'"
    
    # - '-lossless 0' specifies a lossy conversion (set to '1' for lossless)
    # - '-qscale 80' sets the quality (0–100, where lower numbers mean better quality but larger size)
    ffmpeg -y -i "$file" -lossless 0 -qscale 80 "$output"
    
    if [ $? -eq 0 ]; then
        echo "Conversion successful: $output"
    else
        echo "Error converting: $file" >&2
    fi
done

