#!/bin/sh

rm -r /tmp/roger/

PREVIOUS=""
while true; do
	rsync -a fuzz/corpus/roger/ /tmp/roger/
	CURRENT="$(stat -c %Y fuzz/corpus/roger)"
	if [ "${CURRENT}" != "${PREVIOUS}" ]; then
		echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
		echo "Changes detected: processing..."
		echo
		./util/htmlcov.sh /tmp/roger
		PREVIOUS=${CURRENT}
		echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
		echo "Waiting for changes..."
		echo
	fi
	# Sleeping is not ideal, but the coverage script
	# takes around 3 seconds to run, so a one second
	# maximum additional delay is okay here
	sleep 1
done
