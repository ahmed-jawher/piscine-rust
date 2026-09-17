use json;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let cals_text = &food.calories.1;
        let cals_value = cals_text
            .trim_end_matches("kcal")
            .parse::<f64>()
            .unwrap_or(0.0);

        total_cals += cals_value * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    let rounded_cals = (total_cals * 100.0).round() / 100.0;
    let rounded_carbs = (total_carbs * 100.0).round() / 100.0;
    let rounded_proteins = (total_proteins * 100.0).round() / 100.0;
    let rounded_fats = (total_fats * 100.0).round() / 100.0;

    json::object! {
        "cals": rounded_cals,
        "carbs": rounded_carbs,
        "proteins": rounded_proteins,
        "fats": rounded_fats
    }
}
