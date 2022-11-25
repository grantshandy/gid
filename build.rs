use std::{env, fs, io};

const NAME: &str = "clientsecret";

fn main() -> io::Result<()> {
    let mut dest = env::var("OUT_DIR").unwrap().to_string();
    dest.push_str(&format!("/{NAME}"));

    let data = base64::encode(fs::read(format!("{NAME}.json"))?);

    fs::write(dest, data)?;

    Ok(())
}
