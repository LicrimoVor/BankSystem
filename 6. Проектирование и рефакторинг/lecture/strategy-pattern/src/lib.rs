pub trait SortingStrategy {
    fn sort(&self, data: &mut [i32]);
}

pub struct QuickSort;
pub struct MergeSort;

impl SortingStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        data.sort();
        println!("Using QuickSort");
    }
}

impl SortingStrategy for MergeSort {
    fn sort(&self, data: &mut [i32]) {
        data.sort();
        println!("Using MergeSort");
    }
}
