use regex::Regex;

#[test]
fn get_version_should_print_xfce_version() {
    let regex = Regex::new(r"^4\.(16|18|20)$").unwrap();
    let version = libxfce4util::version_string();
    if !regex.is_match(version.as_str()) {
        panic!("Invalid Xfce version: {}", version);
    }
}
