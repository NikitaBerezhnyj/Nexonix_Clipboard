# Nexonix Clipboard

Nexonix Clipboard repository has [Ukrainian :ukraine:](#nexonix-clipboard-ukraine) and [English :uk:](#nexonix-clipboard-uk) localizations

## Nexonix Clipboard :ukraine:

<p align='center'>
  <img src='./src-tauri/icons/128x128.png' alt='Іконка застосунку' style="width:50%">
</p>

Цей проект реалізує розширений буферу обміну для операційної системи з використанням Tauri. Він дозволяє користувачам зберігати історію буферу обміну, керувати нею та взаємодіяти через системний tray.

### Особливості

- **_Збереження історії буферу обміну:_** Зберігає до 5 останніх записів у буфері обміну.
- **_Інтерфейс системного tray:_** Надає можливість взаємодії з історією буферу обміну через меню системного tray.
- **_Автоматичне прослуховування:_** Автоматично зберігає текст, скопійований до буферу обміну, якщо активовано прослуховування.
- **_Керування прослуховуванням:_** Можливість зупинити та відновити прослуховування буферу обміну.
- **_Очищення історії:_** Можливість очищення всієї історії буферу обміну.

### Встановлення

1. Клонування репозиторію

```bash
git clone https://github.com/NikitaBerezhnyj/Nexonix_Clipboard.git
```

2. Перейдіть до теки проекту

```bash
cd nexonix-clipboard
```

3. Встановлення залежностей

Переконайтеся, що у вас встановлений Rust та Tauri CLI.

bash

```bash
cargo install tauri-cli
```

4. Запуск додатку

```bash
cargo tauri dev
```

### Використання

Після запуску додатку, він буде працювати у фоновому режимі та з'явиться в системному tray. Ви можете виконувати наступні дії через меню tray:

- **_Pause listening / Listen:_** Зупинити або відновити прослуховування буферу обміну.
- **_Clear:_** Очистити історію буферу обміну.
- **_Quit:_** Вийти з додатку.

## Nexonix Clipboard :uk:

<p align='center'>
  <img src='./src-tauri/icons/128x128.png' alt='Іконка застосунку' style="width:50%">
</p>

This project implements an advanced clipboard for the operating system using Tauri. It allows users to store, manage, and interact with clipboard history through the system tray.

### Features.

- **_Save clipboard history:_** Saves up to 5 most recent entries on the clipboard.
- **_System tray interface:_** Provides the ability to interact with the clipboard history through the system tray menu.
- **_Auto Listening:_** Automatically saves text copied to the clipboard when listening is enabled.
- **_Listening control:_** Pause and resume listening to the clipboard.
- **_Clear history:_** Clear the entire history of the clipboard.

### Installation.

1. Cloning the repository

```bash
git clone https://github.com/NikitaBerezhnyj/Nexonix_Clipboard.git
```

2. Change to the project folder

```bash
cd nexonix-clipboard
```

3. Installing dependencies

Make sure you have Rust and the Tauri CLI installed.

```bash
cargo install tauri-cli
```

1. Running the application

```bash
cargo tauri dev
```

### Usage

After launching the application, it will run in the background and appear in the system tray. You can perform the following actions through the tray menu:

- **_Pause listening:_** Pause or resume listening to the clipboard.
- **_Clear:_** Clear the clipboard history.
- **_Quit:_** Exit the application.
