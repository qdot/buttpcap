# ButtPCap

[![Patreon donate button](https://img.shields.io/badge/patreon-donate-yellow.svg)](https://www.patreon.com/qdot)
[![Github donate button](https://img.shields.io/badge/github-donate-ff69b4.svg)](https://www.github.com/sponsors/qdot)
[![Discord](https://img.shields.io/discord/353303527587708932.svg?logo=discord)](https://discord.buttplug.io)

[![Twitter](https://img.shields.io/twitter/follow/buttplugio.svg?style=social&logo=twitter)](https://twitter.com/buttplugio)
[![Twitter](https://img.shields.io/twitter/follow/qdot.svg?style=social&logo=twitter)](https://twitter.com/qdot)

[![Youtube: Buttpluggin' With qDot](https://img.shields.io/youtube/channel/subscribers/UCKLmntfj3NmM2E3_LB3qs-w?label=Buttpluggin%27%20With%20qDot&style=social)](https://youtube.buttplug.io)
[![Youtube: Poor Life Choices](https://img.shields.io/youtube/channel/subscribers/UCEOH7Ne1LflFosQTpzM0ZrA?label=Poor%20Life%20Choices&style=social)](https://www.youtube.com/channel/UCEOH7Ne1LflFosQTpzM0ZrA)


Using [USBPcap](https://github.com/desowin/usbpcap/) to side-step anticheat in games, in order to
reroute rumble packets to sex toys via [The Buttplug Sex Toy Control Library](https://buttplug.io).

## Demo Video

There's a [demo video on the Buttpluggin' With qDot Youtube Channel](https://www.youtube.com/watch?v=KyMZBOQtmic).

## WARNING

Do not actually use this. This is a shitpost project I made to work around the fact that games using EasyAntiCheat and other anti cheat products don't work with the [Game Haptics Router](https://intiface.com/ghr).

This project requires you to install a USB Packet Filter, which means EVERY SINGLE THING that goes over your USB bus, including keystrokes from your keyboard, can be picked up by any program with admin privileges.

I cannot stress how fucking stupid this is. Do not use this.

But if you really want to anyways, it's your funeral, here's the code.

## Current Status

The code now works with the stock USBPcap driver from the binary install, as well as stock USBPcapCMD. We assume these are in `c:\Program Files\USBPcap`.

The system does automated lookup for the device index of an Xbox One Controller. This is *required* to be an actual Xbox One Controller or something that registers with that exact USB name. If you have other controllers you think would work with this, let me know in the issues on this repo and I can add a lookup array for checking. I don't get VID/PID currently so we're stuck with name testing.

## Compiling and Installation

Coming Soon

## How It Works

Coming Soon

## License

Licensed under BSD 3-Clause

```
Copyright (c) 2022, Kyle Machulis
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

* Neither the name of buttplug nor the names of its
  contributors may be used to endorse or promote products derived from
  this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```