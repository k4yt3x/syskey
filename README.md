# Motorola System Key Generation Utility

> **Warning**\
> Please use this tool only for the system(s) you are authorized to program radio apparatus for.\
> Illegal use could lead to legal consequences.

This program is a Rust rewrite of Motorola's system key generation utility originally written for MS-DOS. The purpose of the rewrite is to make this utility compatible with the modern computer architectures so it does not need to be executed in MS-DOS emulators such as DOSBox. It is also cross-platform so it can be compiled and used on all Windows, Linux, and macOS.

This software is made with publicly-available information from [BatLabs](http://www.batlabs.com/syskey.html).

![Windows](https://user-images.githubusercontent.com/21986859/194345080-7075ff48-d23c-4c1c-995c-e7ebda03c40c.png)

## Usages

You can download the released executable from [releases](https://github.com/k4yt3x/syskey/releases/latest).

There are two ways to run the software. You can either specify the SysID in the command line like `./syskeygen 1234` or enter the SysID after starting the program with no arguments. The key file will be generated under the same directory the generator's binary is in and will be named `SYS{SysID}.KEY`, such as `SYS01234.KEY`. The screenshot at the top of this page shows how to launch it interactively in Widnows. Below is how to launch it in Linux with the SysID passed as an argument:

![Linux](https://user-images.githubusercontent.com/21986859/194345387-8ca64321-db69-4740-8416-2a7b807ad714.png)

After you get the system key file, proceed to the CPS and load the key by clickling on `Tools > System Key > Load Software Key(s)` and select the key that was generated. Then, you should be able to program your radio for that SysID.

![LoadKey](https://user-images.githubusercontent.com/21986859/194345876-7c2d8670-16a2-437c-b9c8-26aefeb106fb.png)

> **Note**: MOTOROLA is a registered trademark of Motorola Trademark Holdings, LLC.
