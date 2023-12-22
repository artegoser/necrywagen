use regex::Regex;
use std::collections::HashSet;

pub fn is_intersect(adresses: &HashSet<String>, keys: &(String, String, String)) -> bool {
    adresses.contains(&keys.0) || adresses.contains(&keys.1)
}

pub fn is_match(re: &Regex, keys: &(String, String, String)) -> bool {
    re.is_match(&keys.0) || re.is_match(&keys.1)
}
