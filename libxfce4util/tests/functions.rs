use libxfce4util::*;
use regex::Regex;

#[test]
fn get_version_should_print_xfce_version() {
    let regex = Regex::new(r"^4\.(16|18|20)$").unwrap();
    let version = version_string();
    if !regex.is_match(version.as_str()) {
        panic!("Invalid Xfce version: {}", version);
    }
}

#[test]
fn get_license_text_should_return_valid_texts() {
    let bsd = get_license_text(ffi::XFCE_LICENSE_TEXT_BSD);
    let bsd = bsd.trim();
    assert!(bsd.starts_with("Redistribution and use in source and binary forms"));
    assert!(bsd.ends_with("EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."));
    let gpl = get_license_text(ffi::XFCE_LICENSE_TEXT_GPL);
    let gpl = gpl.trim();
    assert!(gpl.starts_with("This program is free software; you can redistribute it"));
    assert!(gpl.ends_with("Street, Fifth Floor, Boston, MA 02110-1301, USA."));
    let lgpl = get_license_text(ffi::XFCE_LICENSE_TEXT_LGPL);
    let lgpl = lgpl.trim();
    assert!(lgpl.starts_with("This library is free software; you can redistribute it"));
    assert!(lgpl.ends_with("Boston, MA 02110-1301  USA."));
}
