<div align="center">

# Kuiper 

[![Builds](https://img.shields.io/github/actions/workflow/status/Redhawk18/kuiper/build.yml)](https://github.com/Redhawk18/kuiper/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/Redhawk18/kuiper)](https://github.com/Redhawk18/kuiper/blob/main/LICENSE)

A blazing fast [Integrated Development Environment](https://en.wikipedia.org/wiki/Integrated_development_environment), meant to give power back to developers.

<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

</div>

## Installation
No package has been created, at this time, packaging is massively helpful to the project.

Currently to install

1. Clone the repository

```
git clone git@github.com:Redhawk18/kuiper.git
```

2. Compile and install the program

```
cargo install --path kuiper
```

3. Add given path to your `$PATH`

## Building

### Known System Dependencies
If youre operating system needs additional packages please create a pull request with them listed. Please raise a issue if the list is wrong or outdated or has unneeded dependencies.
<details>
  <summary>OpenSuse</summary>

  ```
  sudo zypper install atkmm-devel pkg-config rustup
  ```
</details>

1. Clone the repository

```
git clone git@github.com:Redhawk18/kuiper.git
```

2. Go into the repository

```
cd kuiper
```

3. Compiling

```
cargo build --release
```

## Roadmap
[refer here](ROADMAP.md)

## Contribute
A Proper guide has not been made yet, but any are welcomed.
