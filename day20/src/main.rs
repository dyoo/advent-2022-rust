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
}

struct DlistCursor<'a> {
    dlist: &'a mut Dlist,
    index: usize,
}

impl<'a> DlistCursor<'a> {
    fn new(dlist: &'a mut Dlist, index: usize) -> Self {
        Self { dlist, index }
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
        self.index = self.dlist.delete(self.index);
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
}
