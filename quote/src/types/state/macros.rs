/// ### Макрос для создания методов получения guard версий полей мастера
/// я так и не смог разобраться как делать автоматическую документацию (либо у меня IDE не подтягивает)
/// concat! не работает
#[macro_export]
macro_rules! state_accessor {
    // ---------- Mutex ----------
    (
        fn $name:ident,
        id = $id:expr,
        field = $field:ident,
        type = $type:ty,
        sync = Mutex,
    ) => {
        #[doc = "### Получение guard версии поля мастера"]
        pub(crate) fn $name(
            &self,
        ) -> Result<
            ValueGuard<
                std::sync::MutexGuard<'_, $type>,
                Rc<RefCell<Vec<u8>>>,
                impl FnOnce(Rc<RefCell<Vec<u8>>>),
            >,
            QuoteError,
        > {
            #[cfg(feature = "checking")]
            if !self.can_get($id) {
                panic!(concat!(
                    stringify!($name),
                    " берется ",
                    stringify!($id),
                    "-м"
                ));
            } else {
                self.queue.borrow_mut().push($id);
            }

            let value = self
                .state
                .$field
                .lock()
                .map_err(|_| QuoteError::InternalError)?;

            Ok(ValueGuard::new(
                value,
                self.queue.clone(),
                gen_callback($id),
            ))
        }
    };

    // ---------- RwLock ----------
    (
        fn $name:ident,
        fn_mut $name_mut:ident,
        id = $id:expr,
        field = $field:ident,
        type = $type:ty,
        sync = RwLock
    ) => {
        #[doc = "### Получение guard версии поля мастера"]
        pub(crate) fn $name(
            &self,
        ) -> Result<
            ValueGuard<
                std::sync::RwLockReadGuard<'_, $type>,
                Rc<RefCell<Vec<u8>>>,
                impl FnOnce(Rc<RefCell<Vec<u8>>>),
            >,
            QuoteError,
        > {
            #[cfg(feature = "checking")]
            if !self.can_get($id) {
                panic!(concat!(
                    stringify!($name),
                    " берется ",
                    stringify!($id),
                    "-м"
                ));
            } else {
                self.queue.borrow_mut().push($id);
            }

            let value = self
                .state
                .$field
                .read()
                .map_err(|_| QuoteError::InternalError)?;

            Ok(ValueGuard::new(
                value,
                self.queue.clone(),
                gen_callback($id),
            ))
        }

        pub(crate) fn $name_mut(
            &self,
        ) -> Result<
            ValueGuard<
                std::sync::RwLockWriteGuard<'_, $type>,
                Rc<RefCell<Vec<u8>>>,
                impl FnOnce(Rc<RefCell<Vec<u8>>>),
            >,
            QuoteError,
        > {
            #[cfg(feature = "checking")]
            if !self.can_get($id) {
                panic!(concat!(
                    stringify!($name),
                    " берется ",
                    stringify!($id),
                    "-м"
                ));
            } else {
                self.queue.borrow_mut().push($id);
            }

            let value = self
                .state
                .$field
                .write()
                .map_err(|_| QuoteError::InternalError)?;

            Ok(ValueGuard::new(
                value,
                self.queue.clone(),
                gen_callback($id),
            ))
        }
    };
}
