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
}

struct DlistCursor<'a> {
    dlist: &'a Dlist,
    index: usize,
}

impl<'a> DlistCursor<'a> {
    fn new(dlist: &'a Dlist, index: usize) -> Self {
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
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_forward() {
        let dlist = Dlist::new([3, 1, 4]);
        let mut cursor = DlistCursor::new(&dlist, 0);
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
        let dlist = Dlist::new([3, 1, 4]);
        let mut cursor = DlistCursor::new(&dlist, 0);
        assert_eq!(3, cursor.val());
        cursor.back();
        assert_eq!(4, cursor.val());
        cursor.back();
        assert_eq!(1, cursor.val());
        cursor.back();
        assert_eq!(3, cursor.val());
    }
}
