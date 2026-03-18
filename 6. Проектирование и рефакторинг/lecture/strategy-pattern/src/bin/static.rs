use strategy_pattern::{QuickSort, SortingStrategy};

struct Sorter<S: SortingStrategy> {
    strategy: S,
}

impl<S: SortingStrategy> Sorter<S> {
    fn new(strategy: S) -> Self {
        Self { strategy }
    }

    fn sort(&self, data: &mut [i32]) {
        self.strategy.sort(data);
    }
}

fn main() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];

    let quick_sorter = Sorter::new(QuickSort);
    quick_sorter.sort(&mut data);

    println!("Sorted: {:?}", data);
}
