#![windows_subsystem = "windows"]
mod ui;
mod util;

use fltk::app;
use fltk_theme::{ThemeType, WidgetTheme};
use std::{env::{self, args}, fs, path::Path, process::exit};

fn main() {
    let app = app::App::default();
    WidgetTheme::new(ThemeType::Metro).apply();
    
    let args: Vec<String> = args().into_iter().collect();
    if args.len() < 2 {
        let execute = env::current_exe().unwrap();
        let execute_path = execute.as_path();
        update_reg(execute_path);
        ui::dialog::message("Menu has been created.\n Right click any file then select 'FileTag' menu item.", "Close");
        exit(0);
    }

    if args[1] == "--uninstall" {
        delete_reg();
        ui::dialog::message("Menu has been deleted.", "Close");
        exit(0);
    }

    let path = Path::new(&args[1]);
    let name = util::CanonicalName::new(String::from(path.file_name().unwrap().to_str().unwrap()));
    let new_file_name = if name.is_canonical() {
        name.to_upgrade_string()
    } else {
        name.to_string()
    };

    let choice = ui::dialog::choice(
        &format!(
            "Copy '{}' \n to '{}' ?",
            &name.get_original(),
            &new_file_name
        ),
        "Ok",
        "Cancel",
    );

    if !choice {
        exit(0)
    }

    let to = path.parent().unwrap().join(new_file_name.to_string());
    if to.exists() {
        ui::dialog::message(
            &format!("Copy failed, \n'{}' exists.", &new_file_name),
            "Close",
        );
    } else {
        match fs::copy(path, to) {
            Ok(_) => (),
            Err(e) => {
                ui::dialog::message(&format!("Copy failed, {}.", e), "Close");
            }
        }
    }

    app.run().unwrap();
}

fn update_reg(path: &Path) {
    let hkey = winreg::enums::HKEY_CURRENT_USER;
    let file_tag_path = Path::new("Software\\Classes\\*\\shell\\FileTag");
    let command_path = Path::new("Software\\Classes\\*\\shell\\FileTag\\command");

    util::reg::set_value(hkey, file_tag_path, "", &"FileTag").expect("Failed update registry.");
    util::reg::set_value(
        hkey,
        command_path,
        "",
        &format!("{} %1", &path.to_str().unwrap()),
    )
    .expect("Failed update registry.");
}

fn delete_reg() {
    let hkey = winreg::enums::HKEY_CURRENT_USER;
    let file_tag_path = Path::new("Software\\Classes\\*\\shell\\FileTag");
    util::reg::delete_subkey_all(hkey, file_tag_path).expect("Failed delete registry.");
}
