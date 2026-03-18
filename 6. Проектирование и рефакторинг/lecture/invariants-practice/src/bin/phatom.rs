use std::marker::PhantomData;

struct BorrowedSlice<'a, T> {
    data: *const T,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T> BorrowedSlice<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Self {
            data: data.as_ptr(),
            len: data.len(),
            _marker: PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Option<&'a T> {
        if index < self.len {
            unsafe {
                let ptr = self.data.add(index);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn iter(&self) -> BorrowedSliceIter<'_, T> {
        BorrowedSliceIter {
            slice: self,
            index: 0,
        }
    }
}

struct BorrowedSliceIter<'a, T> {
    slice: &'a BorrowedSlice<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for BorrowedSliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slice.len() {
            let item = self.slice.get(self.index);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let slice = BorrowedSlice::new(&data);

    for item in slice.iter() {
        println!("{}", item);
    }

    println!("Length: {}", slice.len());
}
