Ripto
=====

Ripto is a small desktop GUI that displays cryptocurrency prices (in Toman) using a Slint UI and a Rust backend.

Quick links
-----------

- **Binary (.deb)**: release/ripto_0.1.0_amd64.deb
- **Tarball**: release/ripto-0.1.0-linux-amd64.tar.gz

Contents
--------

- `src/` — Rust source
- `ui/` — Slint UI files (`ui/ui.slint`)
- `icons/` — application icon(s)

Features
--------

- Simple Slint UI window showing multiple coin prices
- Periodic automatic refresh and manual refresh button
- Packaged release artifacts: `.deb` and portable tarball

Build (development)
---------------------

```bash
# install Rust toolchain and Cargo
cargo build
cargo run
```

Build (release)
----------------

```bash
cargo build --release
# release binary: target/release/slint-rust-template
```

Run packaged .deb (system install)
---------------------------------

```bash
# install .deb (Downlaod from release page)
sudo apt install ./ripto_0.1.0_amd64.deb
# then launch from application menu or run:
ripto
```

Install from tarball (portable)
-------------------------------

```bash
tar xzf release/ripto-0.1.0-linux-amd64.tar.gz
# optionally install system-wide:
sudo ./ripto-0.1.0-linux-amd64/INSTALL.sh
```

Icon / KDE notes
----------------

- The app embeds the image referenced from `ui/ui.slint` and the installer places the PNG into the hicolor icon theme.
- On some desktops (GNOME/Wayland/KDE) the window manager may prefer the `.desktop` icon. If the panel/taskbar still shows a generic icon:
  - log out/in or run `kbuildsycoca5` to refresh KDE caches
  - ensure the `.desktop` `StartupWMClass` matches the app's WM_CLASS (I can help determine that if needed)

Packaging details
-----------------

- Debian package created: `release/ripto_0.1.0_amd64.deb`
- Portable release: `release/ripto-0.1.0-linux-amd64.tar.gz` (contains `ripto` binary, `ripto.desktop`, `ripto.png`, and `INSTALL.sh`)

Contributing / Development
--------------------------

- Pull requests and issues are welcome. For development, run the app with `cargo run` and edit `ui/ui.slint` and `src/*.rs`.

License
-------

- See the repository `LICENSE` file at the project root for licensing terms.

If you want, I can also produce a polished GitHub release description and prepare the release assets for upload. If you want uploads automated, provide a GitHub token and target repo and I will prepare the steps.

---

File paths mentioned above are relative to the project root.
# Slint Rust Template

A template for a Rust application that's using [Slint](https://slint.rs/) for the user interface.

## About

This template helps you get started developing a Rust application with Slint as toolkit
for the user interface. It demonstrates the integration between the `.slint` UI markup and
Rust code, how to react to callbacks, get and set properties, and use basic widgets.

## Usage

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`.
2. Download and extract the [ZIP archive of this repository](https://github.com/slint-ui/slint-rust-template/archive/refs/heads/main.zip).
3. Rename the extracted directory and change into it:
    ```
    mv slint-rust-template-main my-project
    cd my-project    
    ```
4. Build with `cargo`:
    ```
    cargo build
    ```
5. Run the application binary:
    ```
    cargo run
    ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

We hope that this template helps you get started, and that you enjoy exploring making user interfaces with Slint. To learn more
about the Slint APIs and the `.slint` markup language, check out our [online documentation](https://slint.dev/docs).

Don't forget to edit this readme to replace it by yours, and edit the `name =` field in `Cargo.toml` to match the name of your
project.
