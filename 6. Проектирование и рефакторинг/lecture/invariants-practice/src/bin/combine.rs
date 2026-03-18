use std::marker::PhantomData;

pub struct SafeArray<'a, T> {
    data: *const T,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T> SafeArray<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        if data.is_empty() {
            panic!("SafeArray cannot be empty");
        }
        Self {
            data: data.as_ptr(),
            len: data.len(),
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn first(&self) -> &'a T {
        unsafe { &*self.data }
    }

    pub fn last(&self) -> &'a T {
        unsafe {
            let ptr = self.data.add(self.len - 1);
            &*ptr
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let array = SafeArray::new(&data);

    println!("First: {}", array.first());
    println!("Last: {}", array.last());
    println!("Length: {}", array.len());
}
