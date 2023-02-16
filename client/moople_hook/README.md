# Proxy DLL to do client modifications

## Building

Either use cross(https://github.com/cross-rs/cross) or build this library locally on a windows machine. I'd recommend setting up a Windows 7/10 VM and use SSH to develop on It remotely.

## Usage instructions

Rename the built `dinput8.dll` to `dinpuz8.dll` in the `target/release` directory and copy It to your Maple Story directory(same directory as your localhost .exe). When you launch the localhost you should see a console being attached.

## Features

* Logging Packet structure tracing data
* Dump the whole string pool
* Catch exceptions before the process is about to crash


## TODO

* Bypass the dinput8.dll check in the ZApiLoader function
* Probably moving away from the proxy dll idea and writing a proper launcher