use super::guard::ValueGuard;
use crate::{
    distributor::Distributor, master::Connection, state_accessor, types::error::QuoteError,
};
#[cfg(feature = "logging")]
use log::{info, warn};
use std::{
    cell::RefCell,
    collections::HashMap,
    net::SocketAddr,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

/// Стейт мастера севера
pub(crate) struct MasterState {
    // последовательность: 1
    connections: Mutex<HashMap<SocketAddr, Vec<Connection>>>,
    // последовательность: 2
    distributor: Mutex<Distributor>,

    // последовательность: 3
    shutdown: RwLock<bool>,
    // последовательность: 4
    secret_key: RwLock<String>,
}

impl MasterState {
    pub(crate) fn new(
        connections: HashMap<SocketAddr, Vec<Connection>>,
        distributor: Distributor,
        shutdown: bool,
        secret_key: String,
    ) -> Self {
        Self {
            connections: Mutex::new(connections),
            distributor: Mutex::new(distributor),
            shutdown: RwLock::new(shutdown),
            secret_key: RwLock::new(secret_key),
        }
    }
}

fn gen_callback(numb: u8) -> impl FnOnce(Rc<RefCell<Vec<u8>>>) {
    move |queue: Rc<RefCell<Vec<u8>>>| {
        let mut queue = queue.borrow_mut();
        queue.pop_if(|v| v == &numb);
    }
}

/// думаю сильно переборщил с решением по 15 пункту ревью)))
pub(crate) struct MasterStateShell {
    state: Arc<MasterState>,
    // магические числа
    // последовательность взятых значений
    queue: Rc<RefCell<Vec<u8>>>,
}

impl MasterStateShell {
    pub(crate) fn new(state: Arc<MasterState>) -> Self {
        {
            Self {
                state,
                queue: Rc::new(RefCell::new(Vec::new())),
            }
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            queue: Rc::new(RefCell::new(Vec::new())),
        }
    }

    state_accessor!(
        fn secret_key,
        fn_mut secret_key_mut,
        id = 4,
        field = secret_key,
        type = String,
        sync = RwLock
    );
    state_accessor!(
        fn shutdown,
        fn_mut shutdown_mut,
        id = 3,
        field = shutdown,
        type = bool,
        sync = RwLock
    );
    state_accessor!(
        fn distributor,
        id = 2,
        field = distributor,
        type = Distributor,
        sync = Mutex,
    );
    state_accessor!(
        fn connections,
        id = 1,
        field = connections,
        type = HashMap<SocketAddr, Vec<Connection>>,
        sync = Mutex,
    );

    /// ## Чистая эвристика
    /// логика такая: магические числа диктуют последовательность
    /// если магическое число больше предыдущего, то брать его нельзя
    fn can_get(&self, numb: u8) -> bool {
        let queue = self.queue.borrow();
        let max_value = queue.last().unwrap_or(&0);

        if *max_value == numb {
            panic!("#reentering number")
        }
        numb > *max_value
    }
}

#[cfg(all(test, feature = "checking"))]
mod tests {
    use super::*;

    fn make_state() -> Arc<MasterState> {
        Arc::new(MasterState::new(
            HashMap::new(),
            Distributor::new(),
            false,
            "test".to_string(),
        ))
    }

    #[test]
    fn correct_lock_order_does_not_panic() {
        let state = make_state();
        let shell = MasterStateShell::new(state);

        let _connections = shell.connections().unwrap(); // 1
        let _distributor = shell.distributor().unwrap(); // 2
        let _shutdown = shell.shutdown().unwrap(); // 3
        let _secret = shell.secret_key().unwrap(); // 4
    }

    #[test]
    #[should_panic]
    fn wrong_order_panics_1() {
        let state = make_state();
        let shell = MasterStateShell::new(state);

        let _secret = shell.secret_key().unwrap(); // 4
        let _connections = shell.shutdown_mut().unwrap(); // 3 → panic
    }

    #[test]
    fn wrong_order_panics_2() {
        let result = std::panic::catch_unwind(|| {
            let state = make_state();
            let shell = MasterStateShell::new(state);

            let _secret = shell.distributor().unwrap(); // 2
            let _connections = shell.connections().unwrap(); // 1 → panic
        });
        assert!(result.is_err());
    }
}
