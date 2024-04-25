use xfconf::*;

#[test]
fn get_int_should_get_valid_value() {
    init().expect("Failed to initialize xfconf");
    let channel = Channel::get("xsettings");
    let value = channel.get_int("/Gdk/WindowScalingFactor", -1);
    assert!(value != -1);
    unsafe { shutdown(); }
}
