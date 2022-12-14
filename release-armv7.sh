#!/bin/bash


# readonly TARGET_HOST=pi@raspberrypi
# readonly TARGET_PATH=/home/pi/hello-world
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/sam

cargo build --release --target=${TARGET_ARCH}
# rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
# ssh -t ${TARGET_HOST} ${TARGET_PATH}