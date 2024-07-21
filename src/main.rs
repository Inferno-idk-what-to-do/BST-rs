use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
struct BTree<T: PartialOrd + Clone + Display> {
    data: T,
    left: Option<Box<BTree<T>>>,
    right: Option<Box<BTree<T>>>,
}

impl<T: PartialOrd + Clone + Display> BTree<T> {
    fn new(data: T) -> Self {
        BTree { data, left: None, right: None }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            if let Some(ref mut left_child) = &mut self.left {
                left_child.insert(data);
            } else {
                let new_node = BTree { data, left: None, right: None };
                self.left = Some(Box::new(new_node));
            }
        } else if data > self.data {
            if let Some(ref mut right_child) = &mut self.right {
                right_child.insert(data);
            } else {
                let new_node = BTree { data, left: None, right: None };
                self.right = Some(Box::new(new_node));
            }
        } 
        // no case for == because don't want to add duplicate entries
    }

    fn print(&self) {
        self.print_in_order();
        println!();
    }

    // TODO make priv somehow?
    fn print_in_order(&self) {
        if self.left == None && self.right == None {
            print!("{}, ", self.data);
            return;
        }

        if let Some(left) = &self.left { left.print_in_order(); }
        print!("{}, ", self.data);
        if let Some(right) = &self.right { right.print_in_order(); }
    }

    // TODO finish
    fn delete(&mut self, target: T) {
        if self.data == target {
            if self.left == None {
                let clone = self.clone();
                if let Some(right) = clone.right {
                    self.data = right.data;
                    self.left = right.left;
                    self.right = right.right;
                } else {
                    self.left = None;
                    self.right = None;
                }
            } else {
                
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none() 
    }

    fn min(&self) -> T {
        if let Some(left) = &self.left {
            left.min()
        } else { return self.data.clone(); }
    }

    fn max(&self) -> T {
        if let Some(right) = &self.right {
            right.max()
        } else { return self.data.clone(); }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct MyPair {
    data: i32,
    idx:  usize,
}

impl Display for MyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.data, self.idx)
    } 
}

fn main() {
    let mut btree: BTree<i32>  = BTree::new(5);
    btree.insert(7);
    btree.insert(4);
    btree.insert(9);
    btree.insert(2);
    btree.insert(6);
    println!("{:?}", btree);
    btree.print();

    btree.delete(5);
    println!("{:?}", btree);
    btree.print();

    let mut btree2: BTree<MyPair> = BTree::new(MyPair { data: 5, idx: 0 });
    btree2.insert(MyPair { data: 4, idx: 1 });
    btree2.insert(MyPair { data: 6, idx: 2 });
    btree2.insert(MyPair { data: 7, idx: 3 });
    btree2.print();
    println!("{}", btree2.min());
    println!("{}", btree2.max());
    println!("{}", btree2.is_leaf());
    println!("{}", btree2.left.unwrap().is_leaf());
}
