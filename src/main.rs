use std::error::Error;

#[cfg(feature = "legacy-methods")]
use windows::Win32::Globalization::{GetACP, LOCALE_SISO639LANGNAME, LOCALE_SISO3166CTRYNAME};

#[cfg(feature = "legacy-methods")]
use win32_locale_info::{get_locale_info, get_locale_lcid, get_user_default_locale_name};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(feature = "legacy-methods")]
    println!("Encoding: cp{}", unsafe { GetACP() });

    #[cfg(feature = "legacy-methods")]
    println!("ISO 639: {}", get_locale_info(LOCALE_SISO639LANGNAME),);
    #[cfg(feature = "legacy-methods")]
    println!("ISO 3166: {}", get_locale_info(LOCALE_SISO3166CTRYNAME),);

    #[cfg(feature = "legacy-methods")]
    println!(
        "GetUserDefaultLocaleName: {}",
        get_user_default_locale_name()
    );

    #[cfg(feature = "legacy-methods")]
    println!("Locale from LCID: {}", get_locale_lcid());

    #[cfg(feature = "global-pref")]
    println!(
        "GlobalizationPreferences: {}",
        win32_locale_info::globalization_preference()?
    );

    Ok(())
}
