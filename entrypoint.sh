#!/bin/bash
WORKDIR="/github/workspace"

mkdir -p $WORKDIR/corpus
mkdir -p $WORKDIR/solutions

echo "test=123" >> $GITHUB_OUTPUT
