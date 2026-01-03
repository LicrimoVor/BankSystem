// loop {
//     if *self.shutdown.read().expect("shutdown poisoned") {
//         info!("TCP server shutting down (shutdown=true)");
//         break;
//     }

//     match self.listener.accept() {
//         Ok((stream, _addr)) => {
//             stream
//                 .set_nonblocking(false)
//                 .context("stream.set_nonblocking(false)")?;

//             stream.set_nodelay(true).ok();
//             stream.set_read_timeout(Some(Duration::from_secs(5))).ok();  // ACK wait
//             stream.set_write_timeout(Some(Duration::from_secs(5))).ok(); // task send

//             let q = Arc::clone(&self.task_queue);
//             let shutdown = Arc::clone(&self.shutdown);

//             let h = thread::spawn(move || -> Result<()> {
//                 let res = Self::handle_worker(stream, q, shutdown)
//                     .context("handle_worker")?;

//                 Ok(res)
//             });

//             handles.push(h);
//         }
//         Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//             // нет новых соединений прямо сейчас
//             thread::sleep(Duration::from_millis(50));
//         }
//         Err(e) => {
//             warn!(error = %e, "accept error");
//             thread::sleep(Duration::from_millis(50));
//         }
//     }
// }
