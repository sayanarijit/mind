use anyhow::Result;
use mind::backend::runner::Runner;

fn main() -> Result<()> {
    let mut runner = Runner::new("/home/sayanarijit/mind".into())?;
    runner.run()
}
