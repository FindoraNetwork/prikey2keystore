use std::{env, path::PathBuf};

use anyhow::{bail, Result};
use eth_keystore::{decrypt_key, encrypt_key};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "version")]
struct Opt {
    /// Your need be encrypted private key string.
    #[structopt(short = "k", long)]
    prikey: Option<String>,

    /// The password of encrypted private key file you support.
    #[structopt(short = "p", name = "password", long)]
    passwd: String,

    /// The encrypted private key file you support.
    #[structopt(short = "f", name = "file", long, parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let file: Option<PathBuf> = opt.file;
    let passwd: String = opt.passwd;
    let prikey: Option<String> = opt.prikey;

    if file.is_some() {
        let prikey =
            String::from_utf8(decrypt_key(&file.ok_or(anyhow::format_err!(""))?, passwd)?)?;
        println!("decrypt successful, private key: {:?}", prikey);
    } else if prikey.is_some() {
        encrypt_key(
            "./",
            &mut rand::thread_rng(),
            prikey.ok_or(anyhow::format_err!(""))?.as_bytes(),
            &passwd,
            Some("encryptstore.key"),
        )?;
        println!(
            "encrypt successful, encryptstore store to {:?}",
            env::current_dir()?.join("/encryptstore.key").to_string_lossy()
        );
    } else if file.is_some() && prikey.is_some() {
        bail!("input parameter is error!!!");
    } else if file.is_none() && prikey.is_none() {
        bail!("input parameter is error!!!");
    }
    Ok(())
}
