mod bst;

fn main() {
    let mut b = bst::BTree::new();
    assert!(b.is_bst());

    assert_eq!(0, b.size());
    assert_eq!(None, b.max());
    assert_eq!(None, b.max());
    assert!(b.is_bst());


    b.insert(2);
    assert_eq!(2, *b.max().unwrap());
    assert_eq!(2, *b.min().unwrap());
    assert_eq!(1, b.size());
    assert!(b.is_bst());

    b.insert(3);
    assert_eq!(3, *b.max().unwrap());
    assert_eq!(2, *b.min().unwrap());
    assert_eq!(2, b.size());
    assert!(b.is_bst());

    b.insert(3);
    assert_eq!(3, *b.max().unwrap());
    assert_eq!(2, *b.min().unwrap());
    assert_eq!(2, b.size());
    assert!(b.is_bst());

    assert!(b.contains(2));
    assert!(b.contains(3));
}
