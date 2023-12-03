# opal-kit
opal-kit is an alternative to [sedutil](https://github.com/Drive-Trust-Alliance/sedutil), a tool published by the Drive Trust Alliance to work with TCG OPAL compliant self-encrypting drives. 
While there are improved forks ([example](https://github.com/ladar/sedutil)) of sedutil, the project is basically unmaintained and I find it inconvenient to use.

## Disclaimer
I am just starting to develop this software at the time of writing this (December 2023).
Consider it as highly experimental and keep in mind that it can and will
lock you out of your computer and make your hard disk unusable.

## Requirements
opal-kit will be using the Linux sed-opal library, so it will require a fairly new kernel (I will start with Linux 6.1).
Only support of OPAL 2.0 is planned and I don't know if it is possible to
recognize or support the "downgraded" standards OPALite and Pyrite.

## Features already implemented
* Scan / Query

Note that at this stage of development, command line switches may exist
that are documented but do not actually do anything.

