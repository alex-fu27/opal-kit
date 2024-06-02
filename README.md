# opal-kit
opal-kit is an alternative to [sedutil](https://github.com/Drive-Trust-Alliance/sedutil), a tool published by the Drive Trust Alliance to work with TCG OPAL compliant self-encrypting drives. 
While there are improved forks ([example](https://github.com/ladar/sedutil)) of sedutil, the project is basically unmaintained and I find it inconvenient to use.

I am just starting to develop this software at the time of writing this (June 2024).
It is highly experimental, and with the currently implemented feature set, it is not of much use.
Keep in mind that it can and will lock you out of your computer and make your hard disk unusable.

Only support of OPAL 2.0 is planned and I don't know if it is possible to
recognize or support the "downgraded" standards OPALite and Pyrite.

## Requirements
opal-kit is using the Linux kernel's sed-opal library, so it will require a fairly new kernel (I will start with Linux 6.1).

## Features already implemented
* Scan / Query: `opal list`
* Password Hashing à la sedutil: `opal hash --variant sedutil`

Note that at this stage of development, command line switches may exist
that are documented but do not actually do anything.

## Features still missing for it to be a MVP
Highest priority first:
* Unlocking
* Password setting
* Setup

