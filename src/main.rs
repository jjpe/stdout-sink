//!

use ipc_chan::{Result, Sink};
use std::io::{self, Stdout, Write};

fn main() -> Result<()> {
    let mut sink = Sink::from_toml("ipc-chan.toml")?;
    let cfg = sink.config();

    println!("Listening @ {}:{}", cfg.host, cfg.port);
    loop { match &*sink.recv::<String>()? {
        "kill:stdout-sink" => break,
        string => {
            let mut stdout: Stdout = io::stdout();
            writeln!(stdout, "{}", string)?;
            stdout.flush()?;
        }
    }}

    Ok(())
}


#[cfg(test)]
mod tests {
    use ipc_chan::{Config, Result, Source};
    use std::time::Duration;

    #[test]
    fn send_test_messages() -> Result<()> {
        let cfg = Config {
            host: "127.0.0.1".to_string(),
            port: 10045, // test-specific port
        };
        let mut source = Source::from_config(cfg.clone())?;
        for i in 0 .. 10 {
            source.send(&format!("Hello World! {}", i))?;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

}
