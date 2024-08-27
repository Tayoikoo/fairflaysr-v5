use std::{io, process::Command, sync::mpsc, thread};

fn print_help() {
    println!(
        "
xtask must specify a task to run.

Usage: `cargo xtask <task>`

Tasks:
    run
        Run the gameserver and sdkserver.
    watch
        Watch for changes in the project and restart the servers if any file changes.
"
    );
}

// run gameserver and sdkserver, wait till any of them exit
fn spawn_servers(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        let mut gameserver = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("gameserver")
            .args(if release { vec!["--release"] } else { vec![] })
            .spawn()
            .expect("failed to start gameserver");

        gameserver.wait()?;
        tx1.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        let mut sdkserver = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("sdkserver")
            .args(if release { vec!["--release"] } else { vec![] })
            .spawn()
            .expect("failed to start sdkserver");

        let _ = sdkserver.wait()?;
        tx2.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    let handle3 = thread::spawn(move || {
        let mut dispatch = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("dispatch")
            .args(if release { vec!["--release"] } else { vec![] })
            .spawn()
            .expect("failed to start dispatch");

        let _ = dispatch.wait()?;
        tx.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    rx.recv().expect("failed to receive from channel");

    handle1.join().expect("failed to join gameserver thread")?;
    handle2.join().expect("failed to join sdkserver thread")?;
    handle3.join().expect("failed to join dispatch thread")?;

    Ok(())
}

// watch for changes in the project and restart the servers if any file changes
fn watch(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("watch").arg("-x").arg(format!(
        "xtask run {}",
        if release { "--release" } else { "" }
    ));

    let mut child = cmd.spawn()?;

    child.wait()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(task) = std::env::args().nth(1) else {
        print_help();
        std::process::exit(0);
    };

    let release = std::env::args().any(|arg| arg == "--release");

    match task.as_str() {
        "run" => spawn_servers(release)?,
        "watch" => watch(release)?,
        _ => {
            println!("invalid task: `{task}`, run `cargo xtask` for a list of tasks");
            std::process::exit(1);
        }
    }

    Ok(())
}
