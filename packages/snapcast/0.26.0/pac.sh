#!/bin/sh
ar x snapclient_0.26.0-1_arm64.deb
unzstd control.tar.zst
unzstd data.tar.zst
xz control.tar
xz data.tar
rm snapclient_0.26.0-1_arm64.deb
ar cr snapclient_0.26.0-1_arm64.deb debian-binary control.tar.xz data.tar.xz