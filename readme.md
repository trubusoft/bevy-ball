Collection of useful docs to quickly start developing with bevy using RustRover on Ubuntu.

# Setup

- Install dependencies

  For ubuntu:

  ```
  sudo apt update
  sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
  ```

  > More on this: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

- Add bevy dependency to `Cargo.toml`

- Run `cargo build` and check if compilation succeed

- Enable Query support for RustRover
    - `org.rust.cargo.evaluate.build.scripts`
    - `org.rust.macros.proc`

  > More on this: https://bevy-cheatbook.github.io/setup/editor/jetbrains.html

- Enable Reformat Code & Optimize Import on Save

  Open Setting > Tools > Action on Save and check `Reformat Code` and `Optimize Import`

- Enable dynamic linking for faster compile in development
    - On Run configuration, append `--features bevy/dynamic_linking` inside the command
    - On Settings > Rust > External Linters, add `--features bevy/dynamic_linking` on Additional arguments

  > More on this: https://bevyengine.org/learn/quick-start/getting-started/setup/#dynamic-linking

# Testing

See [test.rs](src/people/tests.rs) for examples.

> More on this: https://bevy-cheatbook.github.io/patterns/system-tests.html

# References

- [Bevy opinionated best practices](https://github.com/tbillington/bevy_best_practices)
