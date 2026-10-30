//! Sample dishes and variants for the mock. Prices are in sums.

use crate::domain::{Dish, DishKind, PortionLimit, ProductionType};

fn dish(id: &str, name: &str, price: u64, category_id: &str) -> Dish {
    Dish {
        id: id.to_string(),
        kind: DishKind::Dish,
        parent_dish_id: None,
        category_id: Some(category_id.to_string()),
        name: name.to_string(),
        size_name: None,
        price,
        variant_count: 0,
        cooking_time_minutes: 0,
        description: String::new(),
        production_type: ProductionType::Piece,
        is_in_go_list: false,
        is_in_stop_list: false,
        portion_limit: PortionLimit::Unlimited,
    }
}

fn variant(id: &str, parent: &Dish, size_name: &str, price: u64) -> Dish {
    Dish {
        id: id.to_string(),
        kind: DishKind::Variant,
        parent_dish_id: Some(parent.id.clone()),
        category_id: None,
        name: format!("{} {}", parent.name, size_name),
        size_name: Some(size_name.to_string()),
        price,
        ..parent.clone()
    }
}

/// Adds a dish together with its variants and keeps `variant_count` honest.
fn with_variants(dishes: &mut Vec<Dish>, mut parent: Dish, sizes: &[(&str, &str, u64)]) {
    parent.variant_count = sizes.len() as u32;
    let variants: Vec<Dish> = sizes
        .iter()
        .map(|(id, size_name, price)| variant(id, &parent, size_name, *price))
        .collect();
    dishes.push(parent);
    dishes.extend(variants);
}

pub fn sample_dishes() -> Vec<Dish> {
    let mut dishes = vec![
        Dish {
            description: "Ташкентский плов с бараниной, жёлтой морковью, нутом и изюмом. Подаётся с салатом ачичук.".into(),
            cooking_time_minutes: 25,
            is_in_go_list: true,
            portion_limit: PortionLimit::Count(40),
            ..dish("dish-plov", "Плов ташкентский", 45_000, "cat-national")
        },
        Dish {
            cooking_time_minutes: 20,
            ..dish("dish-lagman", "Лагман", 38_000, "cat-national")
        },
        Dish {
            is_in_stop_list: true,
            ..dish("dish-manty", "Манты с бараниной", 32_000, "cat-national")
        },
        Dish {
            portion_limit: PortionLimit::Count(5),
            ..dish("dish-naryn", "Нарын", 35_000, "cat-national")
        },
        Dish {
            cooking_time_minutes: 90,
            description: "Томлёное мясо по-бухарски в казане. Готовится под заказ.".into(),
            ..dish("dish-kazan-kebab", "Казан-кебаб", 120_000, "cat-national")
        },
        Dish {
            production_type: ProductionType::WeightBased,
            cooking_time_minutes: 30,
            description: "Цена за 100 г. Вес определяется после приготовления.".into(),
            ..dish("dish-lamb-shashlik", "Шашлык из баранины (на вес)", 18_000, "cat-grill")
        },
        dish("dish-chicken-shashlik", "Шашлык из курицы", 22_000, "cat-grill"),
        Dish {
            is_in_go_list: true,
            ..dish("dish-lulya", "Люля-кебаб", 24_000, "cat-grill")
        },
        Dish {
            cooking_time_minutes: 15,
            ..dish("dish-shurpa", "Шурпа", 30_000, "cat-soups")
        },
        dish("dish-mastava", "Мастава", 26_000, "cat-soups"),
        Dish {
            is_in_stop_list: true,
            ..dish("dish-borsch", "Борщ со сметаной и пампушками", 28_000, "cat-soups")
        },
        dish("dish-caesar", "Цезарь с курицей", 42_000, "cat-salads"),
        Dish {
            portion_limit: PortionLimit::Count(8),
            ..dish("dish-greek", "Греческий салат", 36_000, "cat-salads")
        },
        dish("dish-achichuk", "Ачичук", 15_000, "cat-salads"),
        dish(
            "dish-warm-salad",
            "Тёплый салат с говяжьей вырезкой, печёными овощами и соусом терияки",
            58_000,
            "cat-salads",
        ),
        Dish {
            cooking_time_minutes: 12,
            ..dish("dish-burger", "Чизбургер", 48_000, "cat-burgers")
        },
        dish("dish-club", "Клаб-сэндвич", 39_000, "cat-burgers"),
        Dish {
            is_in_go_list: true,
            ..dish("dish-shawarma", "Шаурма с курицей", 29_000, "cat-burgers")
        },
        dish("dish-omelette", "Омлет с сыром", 24_000, "cat-breakfast"),
        dish("dish-syrniki", "Сырники со сметаной", 27_000, "cat-breakfast"),
        Dish {
            is_in_stop_list: true,
            ..dish("dish-porridge", "Овсяная каша", 18_000, "cat-breakfast")
        },
        dish("dish-medovik", "Медовик", 25_000, "cat-desserts"),
        Dish {
            portion_limit: PortionLimit::Count(3),
            ..dish("dish-cheesecake", "Чизкейк Нью-Йорк", 32_000, "cat-desserts")
        },
        dish("dish-icecream", "Мороженое пломбир", 14_000, "cat-desserts"),
        dish("dish-americano", "Американо", 18_000, "cat-hot-drinks"),
        dish("dish-cappuccino", "Капучино", 24_000, "cat-hot-drinks"),
        dish("dish-cola", "Coca-Cola 0,5 л", 12_000, "cat-cold-drinks"),
        Dish {
            is_in_stop_list: true,
            ..dish("dish-fresh", "Апельсиновый фреш", 28_000, "cat-cold-drinks")
        },
        dish("dish-nuggets", "Наггетсы с картофелем фри", 26_000, "cat-kids"),
        dish("dish-kids-pasta", "Паста с сыром", 22_000, "cat-kids"),
        Dish {
            cooking_time_minutes: 60,
            description: "Утиная грудка су-вид, пюре из батата, вишнёвый соус.".into(),
            ..dish("dish-duck", "Утиная грудка с вишнёвым соусом", 145_000, "cat-chef")
        },
        dish("dish-steak", "Стейк рибай", 260_000, "cat-chef"),
        dish("dish-somsa", "Самса с мясом", 9_000, "cat-national"),
        dish("dish-dimlama", "Димлама", 34_000, "cat-national"),
    ];

    with_variants(
        &mut dishes,
        Dish {
            cooking_time_minutes: 18,
            description: "Томатный соус, моцарелла, свежий базилик.".into(),
            ..dish("dish-margherita", "Пицца Маргарита", 55_000, "cat-pizza")
        },
        &[
            ("var-margherita-25", "25 см", 55_000),
            ("var-margherita-30", "30 см", 69_000),
            ("var-margherita-35", "35 см", 84_000),
        ],
    );
    with_variants(
        &mut dishes,
        Dish {
            cooking_time_minutes: 18,
            is_in_go_list: true,
            ..dish("dish-pepperoni", "Пицца Пепперони", 65_000, "cat-pizza")
        },
        &[
            ("var-pepperoni-30", "30 см", 79_000),
            ("var-pepperoni-35", "35 см", 94_000),
        ],
    );
    with_variants(
        &mut dishes,
        dish("dish-green-tea", "Чай зелёный", 8_000, "cat-tea"),
        &[
            ("var-green-tea-small", "0,5 л", 8_000),
            ("var-green-tea-large", "1 л", 14_000),
        ],
    );
    with_variants(
        &mut dishes,
        dish(
            "dish-lemonade",
            "Лимонад домашний",
            16_000,
            "cat-cold-drinks",
        ),
        &[
            ("var-lemonade-small", "0,4 л", 16_000),
            ("var-lemonade-large", "1 л", 32_000),
        ],
    );
    mark_variant_states(&mut dishes);
    dishes
}

/// Variants get their own stock states so selecting them visibly changes the details.
fn mark_variant_states(dishes: &mut [Dish]) {
    for dish in dishes.iter_mut() {
        match dish.id.as_str() {
            "var-margherita-35" => dish.is_in_stop_list = true,
            "var-pepperoni-30" => dish.portion_limit = PortionLimit::Count(6),
            "var-green-tea-large" => dish.is_in_go_list = true,
            _ => {}
        }
    }
}
