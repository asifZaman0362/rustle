use sqlite3;
use sqlite3::Result;

fn store_guess(word: String) -> Result<()> {
    let conn = sqlite3::open("data.db")?;
    Ok(())
}
