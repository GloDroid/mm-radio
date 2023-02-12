#!/bin/bash -e

OUT_SRC_DIR=$1
OUT_BUILD_ARCH_DIR=$2
OUT_INSTALL_DIR=$3

HAL_DIR=${OUT_INSTALL_DIR}/vendor/bin/hw
RC_DIR=${OUT_INSTALL_DIR}/vendor/etc/init
VINTF_DIR=${OUT_INSTALL_DIR}/vendor/etc/vintf/manifest

mkdir -p ${HAL_DIR}
mkdir -p ${RC_DIR}
mkdir -p ${VINTF_DIR}

cp ${OUT_BUILD_ARCH_DIR}/android.hardware.mm-radio-service ${HAL_DIR}/
cp ${OUT_SRC_DIR}/android.hardware.radio.rc ${RC_DIR}/
cp ${OUT_SRC_DIR}/android.hardware.radio.xml ${VINTF_DIR}/

llvm-strip ${HAL_DIR}/android.hardware.mm-radio-service
