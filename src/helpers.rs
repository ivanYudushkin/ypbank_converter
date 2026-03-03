///Вспомогательная функция по парсингу &str -> `Option<u64>`. Используется в csv_format и txt_format
pub fn parse_u64(value: &str) -> Option<u64> {
    value.trim().parse::<u64>().ok()
}