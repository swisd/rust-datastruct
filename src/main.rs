use std::error::Error;
pub mod datastruct;
pub mod splog;
pub mod evlog;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    crate::datastruct::Error::print(&datastruct::Error::new(255, "Unspecified Error".parse().unwrap()));
    crate::datastruct::Error::cerror(&datastruct::Error::new(255, "Unspecified Error".parse().unwrap()));
    splog::init_db()?;
    let connection = splog::connect_db("records.db");
    splog::ledger(connection.unwrap(), 1500, 2500, "system", "/", splog::LedgerType::W, "system might be unstable due to incompatibilities")?;
    // evlog::evlog_main();
    Ok(())
}
