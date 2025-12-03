#!/bin/bash

./s0-DeleteGeneratedFiles.sh
./s1-BuildGeneratorsAndTools.sh
./s2-RunCodeGenerator.sh
./s3-BuildGeneratedLib-Rust.sh
