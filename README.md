# ruadio-plugins

Audio plugins built with [NIH-plug](https://github.com/robbert-vdh/nih-plug) and my own audio effects library [ruadio](https://github.com/hswangTW/ruadio). Because the VST3 bindings of NIH-plug are licensed under the GPLv3 license, this project is also licensed under the GPLv3 license.

## Current Status

This project is still under development. Currently, each plugin is just for testing purposes and only has a simple UI. They are tested on macOS 14.5. The VST3 plugins work well, but the standalone version often crashes.

## Build plugins

The plugins are built with NIH-plug's "xtask" tool. Use the following command to build the desired plugin:

```shell
cargo xtask bundle <plugin-name> --release
```

One can check the `bundler.toml` file to see the supported plugins.

