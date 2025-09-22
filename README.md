# Pseudobash

**`Pseudobash`** — это легковесная псевдо-оболочка, написанная на Rust, предназначенная для Linux-систем. Проект сочетает в себе мощь Rust с простотой и удобством традиционных Unix-оболочек.

## Особенности

- **Высокая производительность** благодаря Rust
- **Встроенные команды** (`cat`, `echo`, `wc`, `pwd`, `exit`)
- **Поддержка внешних команд** через `PSEUDOBASH_PATH`
- **Поддержка pipe `|`** **и seq** **`;`**
- **Работа в нескольких режимах** через команду `mode`
- **Минимальное количество зависимостей**: `libc`, `anyhow`
- **Работа с окружением**
- **Документируемая архитектура** в `docs`

## Требования

- **Операционная система**: Linux (Ubuntu, Debian, CentOS, etc.)
- **Rust**: версия 1.70.0 или выше
- **Cargo**: система сборки Rust

## Полная установка

### Шаг 1: Установка Rust (если не установлен)

Установите Rust с официального сайта: [https://www.rust-lang.org/](https://www.rust-lang.org/ "Официальный сайт")

### Шаг 2: Склонируйте репозиторий

```bash
git clone https://github.com/Dx-by-Dy/pseudobash.git
```

### Шаг 3: Перейдите в скачаный репозиторий и соберите все исполняемые файлы

```bash
cd ./pseudobash
```

Соберите все дополнительные программы:

```bash
cd utils && find . -name "Cargo.toml" -exec dirname {} \; | xargs -I {} sh -c 'cd {} && cargo build -r --target-dir ../'; cd ../
```

Соберите `pseudobash`:

```bash
cargo build -r --target-dir .
```

### Шаг 4: Создайте `.env` файл с начальным значением `PSEUDOBASH_PATH`:

```bash
echo "PSEUDOBASH_PATH=$PWD/utils/release" > .env
```

### Шаг 5: Запустите `pseudobash`:

```bash
./release/pseudobash
```

Вы увидите приглашение ввода:

```bash
>>>
```

## Тестирование

### Запустите тестирование:

```bash
cargo test -r
```

Вы должны увидеть, что все тесты пройдены успешно

## Пример использования

```bash
>>> cat ./Cargo.toml
[package]
name = "pseudobash"
version = "1.2.1"
edition = "2024"

[dependencies]
anyhow = "1.0.99"
libc = "0.2.175"
>>> 
```

```bash
>>> echo 100 | wc
1 1 4
>>> 
```

```bash
>>> x=ec
>>> y=ho
>>> $x$y 100
100
>>> 
```

```bash
>>> pwd | wc
1 1 22
>>> 
```

```bash
>>> echo 100; echo 200
100
200
>>> 
```

С помощью команды `mode` можно изменить режим, например, режим `x` где pipe `|` подставляет выход предыдущей команды как дополнительные аргументы:

```bash
>>> mode +x
(x) >>> echo 100 | echo 200
200 100
(x) >>> mode -x
>>> 
```

Также существует режим `i` для работы с интерактивными программами:

```bash
>>> mode +i
(i) >>> /usr/bin/python
Python 3.13.7 (main, Aug 14 2025, 00:00:00) [GCC 15.2.1 20250808 (Red Hat 15.2.1-1)] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> print("Hello")
Hello
>>> exit()
(i) >>> mode -i
>>> 
```

## Планы развития

* **Перенаправление ввода/вывода** (`>`, `<`, `>>`)
* **Работа с терминалом в raw-режиме**
* **Конфигурационный файл** (`~/.pseudobashrc`)
* **Поддержка скриптов**
