use std::env::temp_dir;
use std::fs;
use anyhow::Result;


fn main() -> Result<()> {
    let content1 = fs::read_to_string("./Cargo.toml")?;
    let content2 = fs::read_to_string("./Cargo.lock")?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    let buf = temp_dir();
    fs::write(&buf, &yaml1)?;
    fs::write(&buf, &yaml2)?;

    println!("{}", yaml1);
    println!("{}", yaml2);


    Ok(())

}


fn toml2yaml(content: &str) -> Result<String> {
    let value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}