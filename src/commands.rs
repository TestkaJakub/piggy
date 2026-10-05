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

fn get<'a>(banks: &'a mut Banks, name: &str) -> Result<&'a mut Bank, String> {
    banks.get_mut(name).ok_or_else(|| format!("no piggy bank called '{name}'"))
}

pub fn deposit(banks: &mut Banks, name: &str, amount: &str) -> Result<(), String> {
    let amount = positive_amount(amount)?;
    let b = get(banks, name)?;
    b.balance = b.balance.checked_add(amount).ok_or_else(|| "balance overflow".to_string())?;
    println!("Added {} to '{name}' (now {}).", fmt(amount), fmt(b.balance));
    if let Some(g) = b.goal {
        if b.balance >= g && b.balance - amount < g {
            println!("Goal reached!");
        }
    }
    Ok(())
}

pub fn withdraw(banks: &mut Banks, name: &str, amount: &str) -> Result<(), String> {
    let amount = positive_amount(amount)?;
    let b = get(banks, name)?;
    if amount > b.balance {
        return Err(format!("'{name}' only holds {}", fmt(b.balance)));
    }
    b.balance -= amount;
    println!("Took {} from '{name}' (now {}).", fmt(amount), fmt(b.balance));
    Ok(())
}

pub fn set_goal(banks: &mut Banks, name: &str, amount: Option<&str>) -> Result<(), String> {
    let goal = amount.map(positive_amount).transpose()?;
    get(banks, name)?.goal = goal;
    match goal {
        Some(g) => println!("Goal for '{name}' set to {}.", fmt(g)),
        None => println!("Goal for '{name}' removed."),
    }
    Ok(())
}