# shellcheck shell=sh
exists() {
    command -v "$1" >/dev/null
}

# Prefer the explicitly GNU tools
compat() {
    if exists "$1"; then
        echo "$1"
    elif exists "$1"; then
        echo "$1"
    else
        exit 1
    fi
}
