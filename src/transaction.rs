#[derive(Debug, Clone, PartialEq, Eq)]
//Перечисление типов транзакций
pub enum TransactionType {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL
}
#[derive(Debug, Clone, PartialEq, Eq)]
//Перечисление статусов
pub enum StatusType {
    SUCCESS,
    FAILURE,
    PENDING
}
#[derive(Debug, Clone, PartialEq, Eq)]
//Основная структура в проекте
//Каждий формат парсится к Vec<Transaction> для дальнейшей записи в другой формат или для сравнения
pub struct Transaction {
    pub tx_id: Option<u64>,
    pub tx_type: Option<TransactionType>,
    pub from_user_id: Option<u64>,
    pub to_user_id: Option<u64>,
    pub amount: Option<u64>,
    pub timestamp: Option<u64>,
    pub status: Option<StatusType>,
    pub description: Option<String>
}
//Функция создания новой транзакции
impl Transaction {
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
}