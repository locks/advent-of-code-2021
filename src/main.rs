mod day1;
mod day2;

fn main() {
    let days = vec!["day1", "day2"];
    let choice = dialoguer::Select::new()
        .with_prompt("With day do you want answers for")
        .items(&days)
        .default(days.len() - 1)
        .interact()
        .unwrap();

    match choice {
        0 => {
            day1::run();
        }
        1 => {
            day2::run();
        }
        _ => unreachable!(),
    }
}
