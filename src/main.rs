use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meal {
    name: String,
    tolerance_days: u8,
}

impl Meal {
    fn empty() -> Self {
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
    let mut meals = meals.clone();
    if destination_size < meals.len() {
        return Err(PlanningError::DaysShorterThanMeals);
    }

    // let mut cloned_meals = meals.clone();
    // cloned_meals.sort_by(|a, b| b.tolerance_days.cmp(&a.tolerance_days));
    // let original_len = cloned_meals.len();

    // let whole_repetitions = destination_size / original_len;
    // let remainder = destination_size % original_len;

    // cloned_meals.resize_with(destination_size, Default::default);



    let filled_outer_list = fill_list(&meals, 0 as usize, &mut vec![None; destination_size - meals.len()], &mut 0).into_iter()
        .map(|item| -> Meal {
            match item {
                Some(meal) => meal,
                None => Meal::empty(),
            }
        })
        .collect::<Vec<Meal>>();

    // Combine the original and newly filled lists
    meals.extend(filled_outer_list);
    // meals.append(&mut filled_outer_list);

    // for _ in 1..whole_repetitions {
    //     cloned_meals.extend_from_within(0..original_len);
    // }

    // cloned_meals.extend_from_within(0..remainder);

    Ok(meals)


/*
 * Fill List
 * Fills sublist with meals, appropriately spaced according to tolerance days.
 * The sublist is the lengthened part of the original list, extended to the
 * desired final size.
 */
pub fn fill_list(meals: &Vec<Meal>, iterator: usize, sub_list: &mut Vec<Option<Meal>>, offset: &mut usize) -> Vec<Option<Meal>> {
    if iterator == meals.len() {
        // Break out of iteration
        return sub_list.to_vec();
    }
    let meal = &meals[iterator];
    let max_multiplier = (sub_list.len() as u8 + meals.len() as u8) / meal.tolerance_days;  // Get how many times we want to multiply

    let indexing_function = |i: &u8, new_offset: &usize| (((meal.tolerance_days * i) - meals.len() as u8) + *new_offset as u8) as usize;

    for i in 0..max_multiplier {
        let mut index = indexing_function(&i, &offset);

        if sub_list[index] == None {
            // Look for an open space
            for n in index + 1..sub_list.len() {
                index = indexing_function(&i, &n);
                if sub_list[index] == None {
                    *offset = n;
                    break;
                }
            }
        }

        sub_list[index] = Some(meals[i as usize].clone());
    }

    fill_list(meals, iterator + 1, sub_list, offset)
}

fn generate_meals() -> Vec<Meal> {
    vec![
        Meal {
            name: String::from("Beef Stroganoff"),
            tolerance_days: 3,
        }, Meal {
            name: String::from("PB&J"),
            tolerance_days: 1,
        }// , Meal {
        //     name: String::from("Ham Sandwich"),
        //     tolerance_days: 2,
        // }, Meal {
        //     name: String::from("Hamburger"),
        //     tolerance_days: 2,
        // }, Meal {
        //     name: String::from("Bacon, Eggs, Toast"),
        //     tolerance_days: 3,
        // }, Meal {
        //     name: String::from("Arroz con Pollo"),
        //     tolerance_days: 4,
        // }, Meal {
        //     name: String::from("Scrambled Eggs"),
        //     tolerance_days: 1,
        // }
    ]
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
