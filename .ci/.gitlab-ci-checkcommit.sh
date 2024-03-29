#! /usr/bin/env bash

set -e

echoerr() {
	printf "ERROR: %s\n" "$*" >&2
}

findtag() {
	local commit_body tag person
	commit_body=$1
	tag=$2
	person=$3

	# trim duplicate spaces from commit body and person
	match="$tag: $(echo $person | tr -s ' ')"

	if [ -z "$(echo "$commit_body" | tr -s ' ' | grep -i "$match")" ]; then
		echoerr "Tag is missing from commit body"
		echoerr ""
		echoerr "Looking for '"$match"' in: "
		echoerr "-----------------------------------------------------"
		echoerr "$commit_body"
		echoerr "-----------------------------------------------------"
		echoerr ""
		return 0
	fi

	return 1
}

git fetch https://github.com/GloDroid/mm-radio.git

git log --pretty='%h' FETCH_HEAD..HEAD | while read h; do
	subject=$(git show -s --pretty='%s' "$h")
	if [[ $subject != mm-radio:* ]]; then
		echoerr "Invalid subject prefix: $subject"
		exit 1
	fi

	commit_body=$(git show -s --pretty=%b "$h")

	author=$(git show -s --format='%an <%ae>' "$h")
	if findtag "$commit_body" "Signed-off-by" "$author"; then
		echoerr "Author SoB tag is missing from commit $h"
		exit 1
	fi

	committer=$(git show -s --format='%cn <%ce>' "$h")
	if findtag "$commit_body" "Signed-off-by" "$committer"; then
		echoerr "Committer SoB tag is missing from commit $h"
		exit 1
	fi
done
