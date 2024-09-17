use std::{io::Error, path::Path};

use types::{FromRegValue, ToRegValue};
use winreg::*;

pub fn get_value<T: FromRegValue>(hkey: HKEY, path: &Path, name: &str) -> Result<T, Error> {
    match RegKey::predef(hkey).open_subkey(path) {
        Ok(subkey) => subkey.get_value(name),
        Err(e) => Err(e),
    }
}

pub fn set_value<T: ToRegValue>(
    hkey: HKEY,
    path: &Path,
    name: &str,
    value: &T,
) -> Result<(), Error> {
    match RegKey::predef(hkey).create_subkey(path) {
        Ok((subkey, _)) => subkey.set_value(name, value),
        Err(e) => Err(e),
    }
}

pub fn delete_subkey_all(hkey: HKEY, path: &Path) -> Result<(), Error> {
    RegKey::predef(hkey).delete_subkey_all(path)
}

#[test]
fn test_reg() {
    let hkey = enums::HKEY_CURRENT_USER;
    let path = Path::new("Software\\Classes\\*\\shell\\reg_test");
    let name = "foo";
    let value = "foobar";

    if let Ok(val) = get_value::<String>(hkey, path, name) {
        assert_eq!(&val, value);
        delete_subkey_all(hkey, path).expect(&format!(
            "Failed to delete subkey {} .",
            path.to_str().unwrap()
        ));
    }

    set_value::<String>(hkey, path, name, &value.to_string()).expect("Failed to create key.");
    let val = get_value::<String>(hkey, path, name).expect("Failed to read key.");
    assert_eq!(&val, value);
    delete_subkey_all(hkey, path).expect(&format!(
        "Failed to delete subkey {} .",
        path.to_str().unwrap()
    ));
}
