# AntOS

# Startup
1) Добавить плаформу `thumbv7em-none-eabihf`, в которой не содержиться OS.
   ```command_line
   rustup target add thumbv7em-none-eabihf
   ```
2) Сборка проекта.
   ```command_line
   cargo build --target thumbv7em-none-eabihf
   ```
