#!/bin/bash
#
# Benchmark Compilation Times
#

if [ ! -e /usr/bin/time ]; then
    echo "/usr/bin/time not found" >&2
    exit 1
fi

# cargo clean
# echo "prebuilding dependencies.."
# cargo build --release -q || exit 1

commits=($(git log --pretty=format:"%h;%s" --reverse |
    grep ";bench:" |
    awk -F";" '{print $1}'
))

echo "|commit|measurement|desc|"
echo "|------|-----------|----|"

clean() {
    git checkout master
}

trap clean EXIT INT TERM

for commit in "${commits[@]}"; do
    git checkout $commit &> /dev/null || exit 1
    desc=$(git log --pretty=format:"%s" $commit^..$commit)
    desc=${desc#bench: }
    measurement=$(/usr/bin/time -f "%e" cargo build --release -q 2>&1)
    echo "|$commit|$measurement|$desc|"
done

