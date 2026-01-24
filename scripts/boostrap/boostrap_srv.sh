#!/bin/bash

# Remove old server directories if they exist
for port in 9001 9002 9003 9004; do
    dir="srv$port"
    if [ -d "$dir" ]; then
        rm -rf "$dir"
        echo "Deleted old directory $dir"
    fi
done

# Create directories and HTML files
for port in 9001 9002 9003 9004; do
    dir="srv$port"
    mkdir -p "$dir"
    echo "<h1>Server $port</h1>" > "$dir/index.html"
done

# Start all servers in background
python3 -m http.server 9001 --directory srv9001 &
python3 -m http.server 9002 --directory srv9002 &
python3 -m http.server 9003 --directory srv9003 &
python3 -m http.server 9004 --directory srv9004 &

echo "All servers running in background on ports 9001-9004"