use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::System::UserProfile::GlobalizationPreferences;
use windows::Win32::Globalization::{
    GetLocaleInfoA, GetUserDefaultLocaleName, LOCALE_SISO639LANGNAME, LOCALE_SISO3166CTRYNAME,
    LOCALE_USER_DEFAULT,
};

fn get_locale_info(lctype: u32) -> String {
    let mut buf = vec![0u8; 85];
    let return_code = unsafe { GetLocaleInfoA(LOCALE_USER_DEFAULT, lctype, Some(&mut buf)) };
    buf.truncate(return_code as _);

    String::from_utf8_lossy(&buf).into_owned()
}

fn get_user_default_locale_name() -> String {
    let mut buf = vec![0_u16; 85];
    let return_code = unsafe { GetUserDefaultLocaleName(&mut buf) };
    buf.truncate(return_code as _);
    OsString::from_wide(&buf[..buf.len() - 1])
        .to_string_lossy()
        .into_owned()
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("ISO 639: {}", get_locale_info(LOCALE_SISO639LANGNAME),);
    println!("ISO 3166: {}", get_locale_info(LOCALE_SISO3166CTRYNAME),);
    println!(
        "GetUserDefaultLocaleName: {}",
        get_user_default_locale_name()
    );
    println!(
        "GlobalizationPreferences: {}",
        GlobalizationPreferences::Languages()?.First()?.Current()?
    );

    Ok(())
}
