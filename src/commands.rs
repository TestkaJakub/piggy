use crate::amount::{fmt, positive_amount};
use crate::storage::{Bank, Banks};

pub fn create(banks: &mut Banks, name: &str, goal: Option<&str>) -> Result<(), String> {
    if banks.contains_key(name) {
        return Err(format!("'{name}' already exists"));
    }
    let goal = goal.map(positive_amount).transpose()?;
    banks.insert(name.to_string(), Bank { balance: 0, goal });
    println!("Created '{name}'.");
    Ok(())
}

pub fn delete(banks: &mut Banks, name: &str) -> Result<(), String> {
    let b = banks.remove(name).ok_or_else(|| format!("no piggy bank called '{name}'"))?;
    println!("Deleted '{name}' (it held {}).", fmt(b.balance));
    Ok(())
}