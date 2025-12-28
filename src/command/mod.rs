use std::{process, thread, time::Duration};

use crate::docker;
use tokio::runtime::Runtime;

/// Run the command to start a group
pub async fn run(group: String) {
    info!(target: "lazymc-docker-proxy::command", "Received command to start group: {}", group);
    // Set a handler for SIGTERM
    let cloned_group = group.clone();
    ctrlc::set_handler(move || {
        info!(target: "lazymc-docker-proxy::command", "Received SIGTERM, stopping server...");

        Runtime::new().unwrap().block_on(async {
            docker::stop(cloned_group.clone()).await;
        });
        process::exit(0);
    })
    .unwrap();

    // Start the command
    docker::start(group.clone()).await;

    // Wait for SIGTERM
    loop {
        trace!(target: "lazymc-docker-proxy::command", "Waiting for SIGTERM...");
        thread::sleep(Duration::from_secs(1));
    }
}
