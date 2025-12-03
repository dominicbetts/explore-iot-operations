#!/bin/bash

# Clone the mqtt-patterns repo to generate the code
echo "Clone the iot-operations-sdks repository"
git clone https://github.com/Azure/iot-operations-sdks.git

cd ./iot-operations-sdks/
#git checkout rust/services/v0.7.0
#git checkout -b rust/services/v0.7.0
# git checkout -b commit-69311230 69311230
# git checkout -b commit-43a4053d 43a4053d
git checkout -b commit-92519e15 92519e15

echo "build codegen"
dotnet build -c Release codegen/codegen.sln
cd ..
