mod singlyLinked;
use singlyLinked::Transaction;
//use singlylinked::Transaction;
fn main() {}
#[cfg(test)]
#[test]
fn singlyLinkedList() {
    let mut transaction = Transaction::empty();
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
