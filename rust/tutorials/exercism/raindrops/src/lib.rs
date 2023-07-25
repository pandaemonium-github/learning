
pub fn sound(value: u32) -> String {
    let is_factor = |factor| value % factor == 0;
    let mut result = String::new();
    if is_factor(3) {
        result.push_str("Pling");
    }
    if is_factor(5) {
        result.push_str("Plang");
    }
    if is_factor(7) {
        result.push_str("Plong");
    }
    if result.is_empty() {
        result = value.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_3_as_a_factor_has_sound_with_pling() {
        assert!(sound(3).contains("Pling"));
    }

    #[test]
    fn has_5_as_a_factor_has_sound_with_plang() {
        assert!(sound(5).contains("Plang"));
    }

    #[test]
    fn has_7_as_a_factor_has_sound_with_plong() {
        assert!(sound(7).contains("Plong"));
    }

    #[test]
    fn value_30_has_both_3_and_5_as_factors_sound_pling_plang() {
        assert_eq!(sound(30), "PlingPlang");
    }

    #[test]
    fn value_not_factored_3_5_7_sound_its_value() {
        assert_eq!(sound(34), "34");
    }
}