#![warn(missing_docs)]

//! # ypbank_converter
//!
//! Библиотека для конвертации и сравнения банковских транзакций
//! между форматами YPBankCsv, YPBankText и YPBankBin.
//!
//! ## Поддерживаемые форматы
//!
//! - **CSV** (`.csv`) - текстовый формат с разделителями-запятыми
//! - **Text** (`.txt`) - текстовый формат с ключ-значение парами
//! - **Binary** (`.bin`) - бинарный формат с магическим словом `YPBN`
//! 
//! ## Внутреннее представление транзакций
//! - [`transaction`] - внутренняя структура транзакций
//!
//! ## Основные функции
//!
//! - [`converter::converter`] - конвертация файла из одного формата в другой
//! - [`comparer::comparer`] - сравнение двух файлов на идентичность
///
pub mod converter;
///
pub mod comparer;
///
pub mod transaction;
///
pub mod error; 
///     
pub mod helpers;  
pub mod formats; 
pub use converter::converter;
pub use comparer::comparer;
pub use transaction::{Transaction, TransactionType, StatusType};