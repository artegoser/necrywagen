use std::collections::HashSet;

pub fn is_intersect(adresses: &HashSet<String>, keys: &(String, String, String)) -> bool {
    adresses.contains(&keys.0) || adresses.contains(&keys.1)
}
