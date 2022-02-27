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

What you see here now is just my working MVP version. This required me hacking up parts of USBPcap in order to just get things going quickly. I'll be updating this README and the code as I get things cleaned up, as this project is definitely possible using stock USBPcapCMD. I just wanted to get my shitpost out ASAP.