# Motorola System Key Generation Utility

> **Warning** This software is still in development and is not ready to be used.

This program is a Rust rewrite of Motorola's system key generation utility originally written for MS-DOS. The purpose of the rewrite is to make this utility compatible with the modern computer architectures so it does not need to be executed in MS-DOS emulators such as DOSBox. It is also cross-platform so it can be compiled and used on all Windows, Linux, and macOS.

This software is made with publicly-available information from [BatLabs](http://www.batlabs.com/syskey.html).

![image](https://user-images.githubusercontent.com/21986859/194191216-d15fac9f-6de5-4411-90ad-ca7fa4ee3155.png)

## Usages

There are two ways to run the software. You can either specify the SysID in the command line like `./syskeygen 1234` or enter the SysID after starting the program with no arguments. The key file will be generated under the same directory the generator's binary is in and will be named `{SysID}.key`, such as `1234.key`.

> **Note** MOTOROLA is a registered trademark of Motorola Trademark Holdings, LLC.
