use strategy_pattern::{MergeSort, QuickSort, SortingStrategy};

struct DynamicSorter {
    strategy: Box<dyn SortingStrategy>,
}

impl DynamicSorter {
    fn new(strategy: Box<dyn SortingStrategy>) -> Self {
        Self { strategy }
    }

    fn sort(&self, data: &mut [i32]) {
        self.strategy.sort(data);
    }

    fn change_strategy(&mut self, strategy: Box<dyn SortingStrategy>) {
        self.strategy = strategy;
    }
}

// RUNTIME

fn main() {
    let mut data1 = vec![3, 1, 4, 1, 5];
    let mut data2 = vec![9, 2, 6, 5, 3];

    let mut sorter = DynamicSorter::new(Box::new(QuickSort));
    sorter.sort(&mut data1);

    sorter.change_strategy(Box::new(MergeSort));
    sorter.sort(&mut data2);

    println!("Data1: {:?}", data1);
    println!("Data2: {:?}", data2);
}
