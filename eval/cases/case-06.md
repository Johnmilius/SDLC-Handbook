# Case 06: clean up a shipping calculation

## Plain request

> Clean this up.

## Material (given to both conditions)

The task:

> Clean up `calculate_shipping`. It is hard to read, full of magic numbers, and has no tests.

The function has been in production for two years. There are no tests for it anywhere in the repository.

```rust
// src/shipping.rs

pub fn calculate_shipping(subtotal: f64, weight_kg: f64, country: &str, express: bool) -> f64 {
    let mut cost;
    if country == "US" {
        if subtotal > 50.0 {
            cost = 0.0;
        } else {
            if weight_kg < 1.0 {
                cost = 4.99;
            } else {
                if weight_kg < 5.0 {
                    cost = 8.99;
                } else {
                    cost = 8.99 + (weight_kg - 5.0) * 1.5;
                }
            }
        }
        if express {
            cost = cost + 12.0;
        }
    } else {
        if country == "CA" {
            if weight_kg < 1.0 {
                cost = 9.99;
            } else {
                cost = 9.99 + weight_kg * 2.25;
            }
            if express {
                cost = cost + 20.0;
            }
        } else {
            cost = 24.99 + weight_kg * 4.0;
            if express {
                cost = cost * 1.5;
            }
        }
    }
    (cost * 100.0).round() / 100.0
}
```

The pricing page on the website says: "Free standard shipping on US orders of $50 or more."
