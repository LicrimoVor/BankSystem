use bank::{
    storage::Storage,
    transaction::{Deposit, Transaction, Transfer, Withdraw},
    tx_chain,
};

fn main() {
    let mut storage = Storage::new();
    storage.add_user("Alice".into());
    storage.add_user("Bob".into());

    let tx = tx_chain!(
        Deposit::new("Alice".into(), 500),
        Transfer::new("Alice".into(), "Bob".into(), 50),
        Withdraw::new("Alice".into(), 100)
    );

    // Тип переменной `tx` будет таким:
    //
    // TxCombinator<
    //     Deposit,
    //     TxCombinator<
    //         Transfer,
    //         Withdraw
    //     >
    // >
    //
    // То есть макрос раскладывает цепочку транзакций
    // в дерево вложенных TxCombinator'ов.

    println!("Выполняем транзакции через макрос...");
    match tx.apply(&mut storage) {
        Ok(_) => println!("Успешно"),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    println!("Итоговые балансы:");
    for (name, balance) in storage.get_all() {
        println!("{} -> {}", name, balance);
    }
}
