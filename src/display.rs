use crate::amount::fmt;
use crate::storage::Banks;

fn progress(balance: i64, goal: i64) -> String {
    const WIDTH: i64 = 20;
    let pct = if goal > 0 { balance * 100 / goal } else { 100 };
    let filled = (pct.min(100) * WIDTH / 100) as usize;
    format!(
        "[{}{}] {pct}%",
        "#".repeat(filled),
        "-".repeat(WIDTH as usize - filled)
    )
}

pub fn list(banks: &Banks) {
    if banks.is_empty() {
        println!("No piggy banks yet. Create one with: piggy new <name> [--goal AMOUNT]");
        return;
    }
    let name_w = banks.keys().map(|k| k.chars().count()).max().unwrap_or(0).max(4);
    let bal_w = banks.values().map(|b| fmt(b.balance).len()).max().unwrap_or(0);
    for (name, b) in banks {
        let goal = match b.goal {
            Some(g) => format!("of {:<10} {}", fmt(g), progress(b.balance, g)),
            None => String::new(),
        };
        println!("{name:<name_w$}  {:>bal_w$}  {goal}", fmt(b.balance));
    }
    let total: i64 = banks.values().map(|b| b.balance).sum();
    println!("{}", "-".repeat(name_w + bal_w + 2));
    println!("{:<name_w$}  {:>bal_w$}", "Total", fmt(total));
}