# rustafx-plugins

Audio plugins built with [NIH-plug](https://github.com/robbert-vdh/nih-plug) and my own audio effects library [rustafx](https://github.com/hswangTW/rustafx). Because the VST3 bindings of NIH-plug are licensed under the GPLv3 license, this project is also licensed under the GPLv3 license.

Currently, the plugins have only been tested on macOS 14.5. The VST3 plugins work well, but the standalone version often crashes.

## Build plugins

The plugins are built with NIH-plug's "xtask" tool. Use the following command to build the desired plugin:

```shell
cargo xtask bundle <plugin-name> --release
```

One can check the `bundler.toml` file to see the supported plugins.

