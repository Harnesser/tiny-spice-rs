#! /usr/bin/env sh

echo ${1} ${2}
name=${1}_${2}

mkdir -p waves/${name}

cargo test \
	--release \
	--no-fail-fast \
	--test ${1} ${2} \
	-- --nocapture --ignored \
	> log/${name}.log

egrep \
	--color=always \
	-a2 \
	"ANALYSIS|LOOPRESULT" \
	log/${name}.log


mkdir -p log/html
egrep \
	--color=always \
	-i "Time|METRIC|shifting|<|LOOPRESULT" \
	log/${name}.log \
	| aha > log/html/${name}.html

exit 0
