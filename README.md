# AntOS

# Startup
1) Добавить плаформу `thumbv7em-none-eabihf`, в которой не содержиться OS.
   ```command_line
   rustup target add thumbv7em-none-eabihf
   ```
2) Установить исходный код `rust` для нашего `core`.
   ```command_line
   rustup component add rust-src
   ```
3) Сборка проекта.
   ```command_line
   cargo build
   ```
   