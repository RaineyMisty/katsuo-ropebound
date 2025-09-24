#!/usr/bin/env bash

DEFAULT_NAME="level1.json"

map_name=""
output_name=""   # just the filename, not path
output=""
assets_dir=""

print_usage() {
    echo "Usage: $0 [mapName] [-n|--name name] [-o|--output json file name]"
}

ensure_json_ext() {
    local val="$1"
    [[ "$val" != *.json ]] && val="${val}.json"
    echo "$val"
}

find_assets_dir() {
    local dir="$PWD"
    while [[ "$dir" != "/" ]]; do
        if [[ -d "$dir/assets" ]]; then
            echo "$dir/assets"
            return
        fi
        dir=$(dirname "$dir")
    done
    # none found → make one in current directory
    mkdir -p "$PWD/assets"
    echo "$PWD/assets"
}

set_defaults() {
    # default map name if not set
    [[ -z "$map_name" ]] && map_name="$DEFAULT_NAME"

    # strip .json for later reuse
    local map_base="${map_name%.json}"

    # default output name if not set
    [[ -z "$output_name" ]] && output_name="${map_base}.json"
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
          -n|--name) map_name=$(ensure_json_ext "$2"); shift 2 ;;
          -o|--output) output_name=$(ensure_json_ext "$2"); shift 2 ;;
          -h|--help) print_usage; exit 0 ;;
          *) 
            [[ -z "$map_name" ]] && map_name=$(ensure_json_ext "$1") || { echo "Unknown arg: $1"; exit 1; }
            shift
            ;;
        esac
    done
}

download_map() {
    url="https://shmul.dev/maps/${map_name}"
    assets_dir=$(find_assets_dir)

    # strip .json to get folder name
    map_base="${map_name%.json}"

    # make the map-specific subfolder
    map_dir="${assets_dir}/${map_base}"

    if [[ -d "$map_dir" ]]; then
        rm -rf "$map_dir"
    fi

    mkdir -p "$map_dir"

    output="${map_dir}/${output_name}"
    tmpfile="${output}.part"

    http_status=$(curl -s -w "%{http_code}" -o "$tmpfile" "$url")

    if [[ "$http_status" -ne 200 ]]; then
        echo "❌ Error: $map_name not found (HTTP $http_status)"
        rm -f "$tmpfile"
        exit 1
    fi

    mv "$tmpfile" "$output"
    echo "✅ Downloaded $map_name → $output"
}

# Cross platform support
sedi() {
  if sed --version >/dev/null 2>&1; then
    # GNU sed
    sed -i "$@"
  else
    # BSD sed (macOS)
    sed -i '' "$@"
  fi
}

replace_base64_with_paths() {
    local file="$1"
    local assets_dir="$2"

    local rel_dir="${assets_dir##*/}"   # just "hello", "level1", etc.

    sedi \
        -e "s|\"tileFG\": *\"data:image/png;base64,[^\"]*\"|\"tileFG\": \"${rel_dir}/tileFG.png\"|" \
        -e "s|\"entity\": *\"data:image/png;base64,[^\"]*\"|\"entity\": \"${rel_dir}/entity.png\"|" \
        "$file"
}

extract_base64_field() {
    local key="$1"
    local file="$2"
    grep -o "\"${key}\": *\"data:image/png;base64,[^\"]*\"" "$file" \
    | sed -E "s/\"${key}\": *\"data:image\/png;base64,//; s/\"$//"
}

relative_to_git_root() {
    local path="$1"
    local git_root

    # find repo root
    git_root=$(cd "$path" && git rev-parse --show-toplevel 2>/dev/null) || {
        echo "❌ Error: no .git found above $path" >&2
        return 1
    }

    # resolve absolute path without realpath
    abs_path=$(cd "$path" && pwd)

    # strip git_root prefix
    echo "${abs_path#$git_root/}"
}

main() {
    parse_args "$@"
    set_defaults
    download_map

    # Extract base64-encoded images into map_dir
    tile_fg=$(extract_base64_field "tileFG" "$output")
    entity=$(extract_base64_field "entity" "$output")

    echo "$tile_fg" | base64 --decode > "${map_dir}/tileFG.png"
    echo "$entity"  | base64 --decode > "${map_dir}/entity.png"

    # Replace embedded base64 paths with relative paths
    rel_map_dir=$(relative_to_git_root "$map_dir")
    replace_base64_with_paths "$output" "$rel_map_dir"

    if command -v python3 >/dev/null 2>&1; then
        python3 -m json.tool "$output" > "${output}.tmp" && mv "${output}.tmp" "$output"
    else
        echo "⚠️ python3 not found, JSON left unformatted"
    fi
}

main "$@"
