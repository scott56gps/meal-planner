#[derive(Debug, Clone)]
struct Meal {
    name: String,
    tolerance_days: u8,
}

fn main() {
    let mut meals = generate_meals();
    meals.sort_by(|a, b| b.tolerance_days.cmp(&a.tolerance_days));
    let extended_meals = extend_meals(&meals, 16);

    // assert_eq!(extended_meals, [1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1]);
    println!("{:?}", extended_meals);
}

fn extend_meals(meals: &Vec<Meal>, destination_size: usize) -> Vec<Meal> {
    let mut cloned_meals = meals.clone();
    let original_len = cloned_meals.len();

    let whole_repetitions = destination_size / original_len;
    let remainder = destination_size % original_len;

    for _ in 1..whole_repetitions {
        cloned_meals.extend_from_within(0..original_len);
    }

    cloned_meals.extend_from_within(0..remainder);

    cloned_meals
}

fn generate_meals() -> Vec<Meal> {
    let meals = vec![
        Meal {
            name: String::from("Beef Stroganoff"),
            tolerance_days: 3,
        }, Meal {
            name: String::from("PB&J"),
            tolerance_days: 1,
        }, Meal {
            name: String::from("Ham Sandwich"),
            tolerance_days: 2,
        }, Meal {
            name: String::from("Hamburger"),
            tolerance_days: 2,
        }, Meal {
            name: String::from("Bacon, Eggs, Toast"),
            tolerance_days: 3,
        }, Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 4,
        }, Meal {
            name: String::from("Scrambled Eggs"),
            tolerance_days: 1,
        }];

    meals
}
