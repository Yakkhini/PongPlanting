# Planting Pong

A ECS driven breakout game, writen in Rust, Bevy engine.

The codes is organized well in this project, which contains most of features in Bevy like Component, Event, System, Plugin, State, UI node, Query, Childs in Entity, Serilaize scene from file, Resource, Asset, etc. Besides, there is also some useful tools like debug inspector or heron physic engine are used for the game.

Additionally, some basic code logic like how to setup and clean up scenes write clearly in source files.

This project's code is friendly for newer to Bevy.

## Install

You can browse codes on Github online, or clone this repo to local. If you just want to play it, Planting Pong is available on AUR. You can install it on Arch Linux with AUR helper:

```bash
yay -S pong-planting
```

## Custom Map

You can custom every level's map. A basic template is in `/usr/share/pong-planting/assets/scenes/0-template.ron` which bricks fill up the full map. You can modify it and save in `$HOME/.local/share/pong-planting/scene/filename`.

Map file name's format is `[1-20].scn.ron`, the number in name decides which level to be override.

## License

Planting Pong is under Mulan PSL v2 License.

```
Copyright (c) 2022 Yakkhini
Planting Pong is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
```
