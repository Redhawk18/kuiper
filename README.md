<div align="center">

# Blaze

[![Builds](https://img.shields.io/github/actions/workflow/status/Redhawk18/code-editor/build.yml)](https://github.com/Redhawk18/code-editor/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/Redhawk18/code-editor)](https://github.com/Redhawk18/code-editor/blob/main/LICENSE)

The programmable hybrid GUI and TUI plugin based IDE.

</div>
<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

## Installation
Proper Windows and MacOs installs have not been made, nor for Linux.

Currently to install

1. Clone the repository

```git clone git@github.com:Redhawk18/blaze.git```

2. Compile and install the program

```cargo install --path blaze```

3. Add given path to your `$PATH`

## Building
Blaze is hardware accelerated, so have the proper drivers installed. Rustup is used, and because of that you will need a c linker.

### Known System Dependencies
Arch Linux
```
sudo pacman -S atkmm cmake fontconfig gcc gdk-3.0 make pkg-config rustup
```

Debian
```
sudo apt install build-essential cmake libgtk-3-dev pkg-config
```

OpenSuse
```
sudo zypper install atkmm-devel gdk-pixbuf-devel gdk-pixbuf-xlib-devel glib2-devel gtk3-devel harfbuzz-devel pkg-config rustup
```

1. Clone the repository

```git clone git@github.com:Redhawk18/blaze.git```

2. Go into the repository

`cd blaze`

3. Compiling

`cargo build --release`

## Roadmap
[refer here](ROADMAP.md)

## Contribute
A Proper guide has not been made yet, but any are welcomed.
