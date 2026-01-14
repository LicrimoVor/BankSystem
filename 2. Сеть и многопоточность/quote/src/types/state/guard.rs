/// Выполняет функцию callback при уничтожении объекта
pub struct ValueGuard<O, I, F>
where
    F: FnOnce(I),
{
    out_value: O,

    /// не знаю почему, но без Option не работает .-.
    inner_value: Option<I>,
    callback: Option<F>,
}

impl<O, I, F> ValueGuard<O, I, F>
where
    F: FnOnce(I),
{
    pub fn new(out_value: O, inner_value: I, callback: F) -> Self {
        Self {
            out_value: out_value,
            inner_value: Some(inner_value),
            callback: Some(callback),
        }
    }

    pub fn get(&self) -> &O {
        &self.out_value
    }

    pub fn get_mut(&mut self) -> &mut O {
        &mut self.out_value
    }
}

impl<O, I, F> Drop for ValueGuard<O, I, F>
where
    F: FnOnce(I),
{
    fn drop(&mut self) {
        if let (Some(cb), Some(val)) = (self.callback.take(), self.inner_value.take()) {
            cb(val);
        }
    }
}
