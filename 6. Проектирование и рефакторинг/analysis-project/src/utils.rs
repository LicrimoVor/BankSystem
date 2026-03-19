/// Обёртка, без которой не выполнено требование `std::io::BufReader<T: std::io::Read>`
#[derive(Debug)]
pub struct RefMutWrapper<'a, T>(pub std::cell::RefMut<'a, T>);
impl<'a, T> std::io::Read for RefMutWrapper<'a, T>
where
    T: std::io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
/// Для `Box<dyn много трейтов, помимо auto-трейтов>`, (`rustc E0225`)
/// `only auto traits can be used as additional traits in a trait object`
/// `consider creating a new trait with all of these as supertraits and using that trait here instead`
pub trait MyReader: std::io::Read + std::fmt::Debug + 'static {}
impl<T: std::io::Read + std::fmt::Debug + 'static> MyReader for T {}
