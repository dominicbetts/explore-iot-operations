#!/bin/bash

echo "Delete iot-operations-sdks and generated files"
rm -rf "./iot-operations-sdks/"

if [ -d "../gen/" ]; then
    rm -r "../gen/"
fi
