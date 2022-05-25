mod doublylinked;
mod singlyLinked;
mod skip_linked;
//use singlylinked::Transaction;
fn main() {}
#[cfg(test)]
#[test]
fn doublyLinkedList() {
    let mut transaction = doublylinked::Transaction::empty_first();
    transaction.append("this is a string".to_string());
    transaction.append("this is a string1".to_string());
    transaction.append("this is a string2".to_string());
    transaction.append("this is a string3".to_string());
    transaction.append("this is a string5".to_string());
    transaction.append("this is a string6".to_string());

    /*
    println!("{}", transaction.length);
    println!("{}", transaction.pop());
    println!("{}", transaction.pop());
    println!("{}", transaction.pop());*/
    let mut pop = transaction.pop();
    let mut iterator = doublylinked::ListIterator::new(pop);
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    // println!("{:?}", iterator.next());
    println!("{:?}", iterator.next_back());
    println!("{:?}", iterator.next_back());
    println!("{:?}", iterator.next_back());
}

#[cfg(test)]
#[test]
fn singlyLinkedList() {
    let mut transaction = singlyLinked::Transaction::empty();
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
