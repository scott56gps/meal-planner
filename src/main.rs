use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meal {
    name: String,
    tolerance_days: u8,
}

impl Default for Meal {
    fn default() -> Self {
        Self { name: "".to_string(), tolerance_days: 0 }
    }
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.tolerance_days)
    }
}

// impl PartialEq for Meal {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.tolerance_days == other.tolerance_days
//     }
// }

impl PartialOrd for Meal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tolerance_days.partial_cmp(&other.tolerance_days)
    }
}

impl Ord for Meal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.tolerance_days >= other.tolerance_days {
            true => std::cmp::Ordering::Greater,
            false => std::cmp::Ordering::Less,
        }
    }
}

#[derive(Debug)]
pub enum PlanningError {
    DaysShorterThanMeals
}

impl Error for PlanningError {}
impl Display for PlanningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanningError::DaysShorterThanMeals => write!(f, "Days to plan for are less than the number of meals"),
        }
    }
}

fn main() {
    let meals = generate_meals();
    match extend_meals(&meals, 16) {
        Ok(extended_meals) => extended_meals.into_iter().for_each(|meal| println!("{}", meal)),
        Err(error) => println!("Caught an error: {:?}", error),
    }
}

pub fn extend_meals(meals: &Vec<Meal>, destination_size: usize) -> Result<Vec<Meal>, PlanningError> {
    if destination_size < meals.len() {
        return Err(PlanningError::DaysShorterThanMeals);
    }

    let mut cloned_meals = meals.clone();
    cloned_meals.sort_by(|a, b| b.tolerance_days.cmp(&a.tolerance_days));
    let original_len = cloned_meals.len();

    let whole_repetitions = destination_size / original_len;
    let remainder = destination_size % original_len;

    cloned_meals.resize_with(destination_size, Default::default);

    for _ in 1..whole_repetitions {
        cloned_meals.extend_from_within(0..original_len);
    }

    cloned_meals.extend_from_within(0..remainder);

    Ok(cloned_meals)
}

fn generate_meals() -> Vec<Meal> {
    vec![
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
        }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desired_size_shorter_than_meals_result_err() {
        let meals = generate_meals();
        let desired_size = meals.len() - 1;
        let result = extend_meals(&meals, desired_size);

        assert!(result.is_err_and(|err| matches!(err, PlanningError::DaysShorterThanMeals)));
    }

    #[test]
    fn elements_are_spaced_by_tolerance_days() {
        let meals = vec![Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 3,
        },
        Meal {
            name: String::from("PB&J"),
            tolerance_days: 1,
        }, Meal {
            name: String::from("Posole"),
            tolerance_days: 4,
        }];

        let desired_size = meals.len() * 2;
        let result = extend_meals(&meals, desired_size).unwrap();

        // let included_in_rest_of_result = |&meal, index_from_which_to_search| result[index_from_which_to_search..].contains(meal);
        println!("{:?}", result);

        assert!(result.iter().zip(result.iter().skip(result.len() / meals.len())).all(|(meal1, meal2)| meal1 == meal2));

        // assert!(result.iter().all(|&meal| included_in_rest_of_result(meal, )))
    }

    #[test]
    fn greater_tolerance_days_greater_ordering() {
        let greater_meal = Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 3,
        };
        let lesser_meal = Meal {
            name: String::from("PB&J"),
            tolerance_days: 1,
        };

        assert!(greater_meal > lesser_meal);
    }

    #[test]
    fn equal_tolerance_days_not_equal_ordering() {
        let meal1 = Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 3,
        };
        let meal2 = Meal {
            name: String::from("PB&J"),
            tolerance_days: 3,
        };

        assert_ne!(meal1, meal2);
    }

    #[test]
    fn same_name_and_tolerance_days_equal_ordering() {
        let meal1 = Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 3,
        };
        let meal2 = Meal {
            name: String::from("Arroz con Pollo"),
            tolerance_days: 3,
        };

        assert_eq!(meal1, meal2);
    }
}
