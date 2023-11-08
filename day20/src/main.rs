trait ToSlice {
    fn to_slice(&self) -> &[i32];
}

struct Dlist {
    vals: Vec<i32>,
    preds: Vec<usize>,
    succs: Vec<usize>,
}

impl Dlist {
    fn new(vals: impl IntoIterator<Item = i32>) -> Self {
        let vals: Vec<i32> = vals.into_iter().collect();
        let n = vals.len();
        Self {
            vals,
            succs: (1..n).chain([0]).collect(),
            preds: [n - 1].into_iter().chain(0..=(n - 1)).collect(),
        }
    }

    fn val(&self, index: usize) -> i32 {
        self.vals[index]
    }

    fn forward(&self, index: usize) -> usize {
        self.succs[index]
    }

    fn back(&self, index: usize) -> usize {
        self.preds[index]
    }

    fn delete(&mut self, index: usize) -> usize {
        let next = self.forward(index);
        let prev = self.back(index);
        self.succs[prev] = next;
        self.preds[next] = prev;
        next
    }

    fn insert(&mut self, insertion: usize, insert_at: usize) -> usize {
        let prev = self.back(insert_at);
        self.succs[prev] = insertion;
        self.preds[insertion] = prev;

        self.succs[insertion] = insert_at;
        self.preds[insert_at] = insertion;

        insert_at
    }
}

struct DlistCursor<'a> {
    dlist: &'a mut Dlist,
    index: usize,
    clip: Option<usize>,
}

impl<'a> DlistCursor<'a> {
    fn new(dlist: &'a mut Dlist, index: usize) -> Self {
        Self {
            dlist,
            index,
            clip: None,
        }
    }

    fn val(&self) -> i32 {
        self.dlist.val(self.index)
    }

    fn forward(&mut self) {
        self.index = self.dlist.forward(self.index);
    }

    fn back(&mut self) {
        self.index = self.dlist.back(self.index);
    }

    fn delete(&mut self) {
        self.clip = Some(self.index);
        self.index = self.dlist.delete(self.index);
    }

    fn insert(&mut self) {
        if let Some(to_insert) = self.clip {
            self.index = self.dlist.insert(to_insert, self.index);
            self.clip = None;
        }
    }
}

impl<'a> Iterator for DlistCursor<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let result = self.val();
        self.forward();
        Some(result)
    }
}

fn encrypt(dlist: &mut Dlist) {
    for i in 0..dlist.vals.len() {
        let mut cursor = DlistCursor::new(dlist, i);
        let n = cursor.val();
        cursor.delete();
        if n > 0 {
            for _ in 0..n {
                cursor.forward();
            }
        } else {
            for _ in 0..(-n) {
                cursor.back();
            }
        }
        cursor.insert();
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_forward() {
        let mut dlist = Dlist::new([3, 1, 4]);
        let mut cursor = DlistCursor::new(&mut dlist, 0);
        assert_eq!(3, cursor.val());
        cursor.forward();
        assert_eq!(1, cursor.val());
        cursor.forward();
        assert_eq!(4, cursor.val());
        cursor.forward();
        assert_eq!(3, cursor.val());
    }

    #[test]
    fn iteration() {
        let mut dlist = Dlist::new([3, 1, 4]);
        let cursor = DlistCursor::new(&mut dlist, 0);
        assert_eq!(cursor.take(6).collect::<Vec<_>>(), vec![3, 1, 4, 3, 1, 4]);

        let cursor = DlistCursor::new(&mut dlist, 1);
        assert_eq!(cursor.take(6).collect::<Vec<_>>(), vec![1, 4, 3, 1, 4, 3]);

        let cursor = DlistCursor::new(&mut dlist, 2);
        assert_eq!(cursor.take(6).collect::<Vec<_>>(), vec![4, 3, 1, 4, 3, 1]);
    }

    #[test]
    fn moving_back() {
        let mut dlist = Dlist::new([3, 1, 4]);
        let mut cursor = DlistCursor::new(&mut dlist, 0);
        assert_eq!(3, cursor.val());
        cursor.back();
        assert_eq!(4, cursor.val());
        cursor.back();
        assert_eq!(1, cursor.val());
        cursor.back();
        assert_eq!(3, cursor.val());
    }

    #[test]
    fn deletion() {
        let mut dlist = Dlist::new([3, 1, 4]);
        let mut cursor = DlistCursor::new(&mut dlist, 0);
        cursor.delete();
        assert_eq!(1, cursor.val());
        cursor.forward();
        assert_eq!(4, cursor.val());
        cursor.forward();
        assert_eq!(1, cursor.val());
        cursor.back();
        assert_eq!(4, cursor.val());
    }

    #[test]
    fn insertion_deletion_manually() {
        let mut dlist = Dlist::new([1, 2, -3, 3, -2, 0, 4]);
        let mut cursor = DlistCursor::new(&mut dlist, 0);
        cursor.delete();
        cursor.forward();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![-3, 3, -2, 0, 4, 2, 1]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 1);
        cursor.delete();
        cursor.forward();
        cursor.forward();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![3, -2, 0, 4, 1, -3, 2]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 2);
        cursor.delete();
        cursor.back();
        cursor.back();
        cursor.back();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![0, 4, 1, 2, 3, -2, -3]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 3);
        cursor.delete();
        cursor.forward();
        cursor.forward();
        cursor.forward();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![4, 1, 2, -2, -3, 0, 3]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 4);
        cursor.delete();
        cursor.back();
        cursor.back();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![1, 2, -3, 0, 3, 4, -2]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 5);
        cursor.delete();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![3, 4, -2, 1, 2, -3, 0]
        );

        let mut cursor = DlistCursor::new(&mut dlist, 6);
        cursor.delete();
        cursor.forward();
        cursor.forward();
        cursor.forward();
        cursor.forward();
        cursor.insert();
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![0, 3, -2, 1, 2, -3, 4]
        );
    }

    #[test]
    fn test_encrypt() {
        let mut dlist = Dlist::new([1, 2, -3, 3, -2, 0, 4]);
        encrypt(&mut dlist);
        let cursor = DlistCursor::new(&mut dlist, 0);
        assert_eq!(
            cursor.take(7).collect::<Vec<_>>(),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }
}
