mod doublylinked;
mod singlyLinked;
use singlyLinked::Transaction;
type singlyTransaction = Transaction;
//use singlylinked::Transaction;
fn main() {
    let mut transaction = doublylinked::Transaction::empty_first();
    transaction.append("this is a string".to_string());
    transaction.append("this is a string1".to_string());

    println!("{}", transaction.length);
    println!("{}", transaction.pop());
    println!("{}", transaction.pop());
    println!("{}", transaction.pop());
}
#[cfg(test)]
#[test]
fn singlyLinkedList() {
    let mut transaction = singlyTransaction::empty();
    transaction.append("this is a string".to_string());

    transaction.append("this is a another string".to_string());
    transaction.append("this is a another string1".to_string());
    transaction.append("this is a another string2".to_string());
    assert_eq!(transaction.length, 4);
    assert_eq!(transaction.pop(), "this is a string".to_string());
    assert_eq!(transaction.pop(), "this is a another string".to_string());
    assert_eq!(transaction.pop(), "this is a another string1".to_string());
    assert_eq!(transaction.pop(), "this is a another string2".to_string());
}
