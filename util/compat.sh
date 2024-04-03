# shellcheck shell=sh
exists() {
    command -v "${1}" >/dev/null
}

# Prefer the explicitly GNU tools
compat() {
    if exists "g${1}"; then
        echo "g${1}"
    elif exists "${1}"; then
        echo "${1}"
    else
        exit 1
    fi
}
