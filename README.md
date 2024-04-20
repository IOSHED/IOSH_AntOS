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
3) Установить загрузчик образа на диск.
   ```command_line
   cargo install bootimage
   rustup component add llvm-tools-preview
   ```
4) Сборка проекта.
   ```command_line
   cargo bootimage
   ```
5) Запуск виртуальной машины `QEMU`.
   ```command_line
   cargo run
   ``` 
   