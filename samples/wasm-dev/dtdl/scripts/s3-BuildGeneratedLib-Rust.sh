# Purpose: Build the Rust library for the generated code

#!/bin/bash
currentPath=$(pwd)

tinykube_mrpc_command_path="../gen/tinykube_mrpc_command"
tinykube_telemetry_path="../gen/tinykube_telemetry"
tinykube_operator_command_path="../gen/tinykube_operator_command"

# build tinykube_mrpc_command
echo "build tinykube_mrpc_command"
cd $tinykube_mrpc_command_path
cargo build

# build tinykube_telemetry
echo "build tinykube_telemetry"
cd $currentPath
cd $tinykube_telemetry_path
cargo build

# build tinykube_operator_command
echo "build tinykube_operator_command"
cd $currentPath
cd $tinykube_operator_command_path
cargo build
