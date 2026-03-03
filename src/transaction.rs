#[derive(Debug, Clone, PartialEq, Eq)]
///Возможные типы транзакций
pub enum TransactionType {
    ///Пополнение
    DEPOSIT,
    ///Перевод
    TRANSFER,
    ///Снятие
    WITHDRAWAL
}
#[derive(Debug, Clone, PartialEq, Eq)]
///Возможные статусы транзакций
pub enum StatusType {
    ///Успех
    SUCCESS,
    ///Ошибка
    FAILURE,
    ///В процессе
    PENDING
}
#[derive(Debug, Clone, PartialEq, Eq)]
///Внутренняя структура транзакций. Каждий формат парсится к `Vec<Transaction>` для дальнейшей записи в другой формат или для сравнения
pub struct Transaction {
    ///Уникальный идентификатор транзакции
    pub tx_id: Option<u64>,
    ///Тип транзакции (DEPOSIT, TRANSFER, WITHDRAWAL)
    pub tx_type: Option<TransactionType>,
    ///ID пользователя-отправителя
    pub from_user_id: Option<u64>,
    ///ID пользователя-получателя
    pub to_user_id: Option<u64>,
    /// Сумма транзакции
    pub amount: Option<u64>,
    ///Время транзакции (timestamp)
    pub timestamp: Option<u64>,
    ///Статус транзакции
    pub status: Option<StatusType>,
    ///Описание транзакции
    pub description: Option<String>
}
impl Transaction {
    ///Функция создания новой транзакции
    pub fn new() -> Self {
        Transaction {
            tx_id: None,
            tx_type: None,
            from_user_id: None,
            to_user_id: None,
            amount: None,
            timestamp: None,
            status: None,
            description: None
        }
    }   
    /// Проверяет, что все поля транзакции заполнены
    pub fn is_complete(&self) -> bool {
        self.tx_id.is_some() &&
        self.tx_type.is_some() &&
        self.from_user_id.is_some() &&
        self.to_user_id.is_some() &&
        self.amount.is_some() &&
        self.timestamp.is_some() &&
        self.status.is_some() &&
        self.description.is_some()
    }
}