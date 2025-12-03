#!/bin/bash

aio_sdk_path="./iot-operations-sdks"
gen="${aio_sdk_path}/codegen/src/Azure.Iot.Operations.ProtocolCompiler/bin/Release/net8.0/Azure.Iot.Operations.ProtocolCompiler"

tinykube_mrpc_command_path="../gen/tinykube_mrpc_command"
tinykube_telemetry_path="../gen/tinykube_telemetry"
tinykube_operator_command_path="../gen/tinykube_operator_command"
mkdir -p $tinykube_mrpc_command_path
mkdir -p $tinykube_telemetry_path
mkdir -p $tinykube_operator_command_path

# generate code
echo "generate rust code from DTDL model"
$gen --modelFile ../model/TinykubeMrpcCommand.json --outDir $tinykube_mrpc_command_path --lang rust
$gen --modelFile ../model/TinykubeTelemetry.json --outDir $tinykube_telemetry_path --lang rust
$gen --modelFile ../model/TinykubeOperatorCommand.json --outDir $tinykube_operator_command_path --lang rust

