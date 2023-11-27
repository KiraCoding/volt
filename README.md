# Revolt

> [!IMPORTANT]\
> Revolt is currently under development, and official releases have not been shipped yet.

> [!CAUTION]\
> This custom client may violate Discords terms of service, which could result in account suspension or termination.\
> The client may also have limited functionality. Use at your own risk.

## Building
All platforms use [wry][WRY_REPO] which depends on the following dependencies for each system.

### Linux
#### Arch Linux / Manjaro:
```sh
sudo pacman -S webkit2gtk-4.1
```

####  Debian / Ubuntu
```sh
sudo apt install libwebkit2gtk-4.1-dev
```

#### Fedora
```sh
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

### macOS
WebKit is native on macOS so everything should be fine.

### Windows
WebView2 provided by Microsoft Edge Chromium is used. Volt supports Windows 7, 8, 10 and 11.

## Contributing
See [`CONTRIBUTING.md`][CONTRIBUTING] for contributing to the project.

## License
Revolt is distributed under the terms of the GNU General Public License v3.0.
- [`LICENSE`][license] ─ `GNU GPL-3.0` ─ https://www.gnu.org/licenses/gpl-3.0.en.html

[LICENSE]: ./LICENSE
[CONTRIBUTING]: ./.github/CONTRIBUTING.md

[RELEASES]: https://github.com/KiraCoding/revolt/releases
[WRY_REPO]: https://github.com/tauri-apps/wry
