# The mm-radio HAL

[![Build Status](https://github.com/GloDroid/mm-radio/actions/workflows/main.yml/badge.svg)](https://github.com/GloDroid/mm-radio/actions)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Discord](https://img.shields.io/discord/753603904406683670.svg?label=Discord&logo=discord&colorB=7289DA&style=flat-square)](https://discord.gg/fmEAJtct)

## Warning!

This project is a free and open-source initiative maintained by a group of volunteers. It is provided "as is" without any warranties or guarantees.
Please be aware that it is a reference proof of concept and the user is fully responsible for any issues that may arise from using the project.

## Introduction

The **mm-radio** project is a Hardware Abstraction Layer (HAL) for the Android operating system that allows communication between the **Modem Manager**
and the **Android** framework. It implements the Android Interface Definition Language (AIDL) radio interface and "wires-up" requests to the
Modem Manager service from the [Mobile broadband connectivity community](https://gitlab.freedesktop.org/mobile-broadband/ModemManager)

It is designed to work on Android 13 and later versions. Older versions do not have AIDL Radio HAL support.

## The implementation details of mm-radio are as follows:

- The `src/radio` directory contains the main binding logic between `IRadio` interface and MM dbus API.
- The `src/mm_zbus` directory contains zbus auto-generated proxies for ModemManager dbus API.
- The `src/utils` directory contains various helpers.

## Building mm-radio using AOSPLESS within a Docker container

To build the mm-radio project using the AOSPLESS approach within a Docker container, you can follow these steps:

1. **Install Docker:** Run the following commands to install Docker on your system:

    ```
    $ sudo apt install docker.io
    $ sudo usermod -aG docker $USER
    ```

- **Note:** Don't forget to reboot your PC to apply group configuration changes.

2. **Clone the project:** Use the following command to clone the mm-radio project:

    ```
    $ git clone https://github.com/GloDroid/mm-radio.git
    $ cd mm-radio
    ```

3. **Build the project:** Run the following command to build the project:

    ```
    $ make ci_fast
    ```

- Or, you can run the following command to build and deploy the project to the target device (assuming that some version of mm-radio is already installed on your device):

    ```
    $ make bd
    ```

**Note:**

- If you encounter permission denied errors when running for the first time, try rebooting your device and running the command again.
- To view mm-radio logs after running `make bd`, export the environment variable `export ADBLOG=1`.

## Embedding the mm-radio project into your build

To embed the mm-radio project into your build, you can use the AOSPEXT make scripts to build mm-radio, Modem Manager, and related repositories.

1. **AOSP Manifest:** An example of AOSP Manifest file including mm-radio, Modem Manager and related repositories can be found at: [Link](https://github.com/GloDroid/glodroid_manifest/blob/a12ae690942c9c5bb7ffc2c2d4790300373f26ac/glodroid.xml#L33)
2. **AOSP product configuration changes:** An example of required AOSP product configuration changes can be found at: [Link](https://github.com/GloDroid/glodroid_device/tree/41c92f73619f3e093ff56b458ad99ff0c76b8401/common/modem)
3. **Modem Manager patches:** The Modem Manager project may require additional patches to compile within AOSP. These patches can be found at: [Link](https://github.com/GloDroid/glodroid_device/tree/359da645595b118a1a9c5918638887b5f7d8696a/patches/vendor/modemmanager)

## Contributing

When contributing to this repository, please follow these guidelines:

1. **Kernel Code of Conduct:** Use the [Kernel Code of Conduct](https://www.kernel.org/doc/html/latest/process/code-of-conduct.html) when submitting patches.  
   (In a simple form just make sure to include a Signed-off-by line in your commits)
2. **Local CI:** Before submitting a patch, ensure that it passes a local CI run by using the `make ci` command.
3. **Commit message:** Keep the commit message clean and describe the changes made in the patch.
4. **Avoid mixing changes:** Avoid mixing bug fixes, refactors, and new features in a single patch. This is considered a bad practice.
5. **Use common prefix for commits:** Use the `mm-radio: ` prefix for commits.

Please make sure that your contributions follow the above guidelines to ensure that the review process is as smooth as possible.

## Project status

Please find the latest status of the project at: [Link](https://github.com/GloDroid/mm-radio/issues/10)
