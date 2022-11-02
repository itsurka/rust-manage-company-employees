
pub fn to_snake_case(s: &String) -> String {
    s.to_lowercase().replace(" ", "_")
}