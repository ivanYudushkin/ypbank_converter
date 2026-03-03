# ypbank_converter

Конвертер и компаратор банковских транзакций для форматов YPBankCsv, YPBankText и YPBankBin.
Проект выполнен в рамках Лабораторной работы №1 курса по Rust в Яндекс Практикум.

## Описание

`ypbank_converter` — Инструмент, который позволяет менять формат данных по банковским транзакциям, а так же сравнивать тва формата на идентичность 

## Поддерживаемые форматы

- **YPBankCsv** 
- **YPBankText** 
- **YPBankBin** 

## Запуск

### Converter

Конвертация файла

```bash
cargo run --bin converter <input_file> <input_format> <output_format>
```

**Параметры:**
- `input_file` — путь к исходному файлу
- `input_format` — формат исходного файла (`csv`, `text`, `bin`)
- `output_format` — целевой формат (`csv`, `text`, `bin`)

**Пример:**
```bash
cargo run --bin converter records.csv csv bin
```

Результат будет сохранен в файл с тем же именем, но с расширением целевого формата (например, `records.bin`).

### Comparer
```bash
cargo run --bin comparer <file1> <format1> <file2> <format2>
```

**Параметры:**
- `file1` — путь к первому файлу
- `format1` — формат первого файла (`csv`, `text`, `bin`)
- `file2` — путь ко второму файлу
- `format2` — формат второго файла (`csv`, `text`, `bin`)

**Пример:**
```bash
cargo run --bin comparer records.csv csv records.bin bin
```

Команда выведет результат сравнения: `TRUE` если файлы идентичны, `FALSE` если различаются.

### CLI
```bash
cargo run
```
**Поддерживаемые команды**

Конвертация файла
```bash
converter <input_file> <input_format> <output_format>
```

Сравнение файлов с транзакциями
```bash
comparer <file1> <format1> <file2> <format2>
```

Выход
```bash
exit
```

## Формат данных транзакции

В этом проекте исходные даннеы преобразуется в векторы транзакций для гибкой конертации данных и для точного сравнения
У транзакции все поля опциональные. Это сделано с целью того, чтобы процесс конвертации всего источника не падал при встрече какой-то "бракованной" транзакции
Каждая транзакция содержит следующие поля (все опциональные):

- `tx_id` — идентификатор транзакции (u64)
- `tx_type` — тип транзакции: `DEPOSIT`, `TRANSFER`, `WITHDRAWAL`
- `from_user_id` — ID отправителя (u64)
- `to_user_id` — ID получателя (u64)
- `amount` — сумма транзакции (u64)
- `timestamp` — временная метка (u64)
- `status` — статус: `SUCCESS`, `FAILURE`, `PENDING`
- `description` — описание транзакции (String)

## Примеры использования

### Конвертация CSV в Binary

```
cargo run --bin converter transactions.csv csv bin
Конвертация прошла успешно
```

### Конвертация Binary в Text

```
cargo run --bin converter transactions.bin bin text
Конвертация прошла успешно
```

### Проверка идентичности файлов

```
cargo run --bin comparer transactions.csv csv transactions.bin bin
====================================================
Результат проверки идентичности двух файлов
Файл1: transactions.csv, Формат: csv
Файл2: transactions.bin, Формат: bin
TRUE
The transaction records in transactions.csv and transactions.bin are identical.
====================================================
```

## Тестирование

Запуск тестов:

```bash
cargo test
```

Тесты проверяют:
- Конвертацию между всеми форматами
- Сравнение файлов в разных форматах
- Корректность чтения и записи данных для разных форматов

## Автор

ivanyudushkin@yandex.ru