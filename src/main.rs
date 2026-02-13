use std::error::Error;

#[cfg(feature = "legacy-methods")]
use windows::Win32::Globalization::{GetACP, LOCALE_SISO639LANGNAME, LOCALE_SISO3166CTRYNAME};

#[cfg(feature = "legacy-methods")]
use win32_locale_info::{
    get_locale_info, get_locale_lcid, get_user_default_locale_name, preferred_ui_languages,
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(feature = "legacy-methods")]
    {
        println!("Encoding: cp{}", unsafe { GetACP() });

        println!("ISO 639: {}", get_locale_info(LOCALE_SISO639LANGNAME),);
        println!("ISO 3166: {}", get_locale_info(LOCALE_SISO3166CTRYNAME),);

        println!(
            "GetUserDefaultLocaleName: {}",
            get_user_default_locale_name()
        );

        println!("Locale from LCID: {}", get_locale_lcid());

        println!("Preferred UI languages:");
        for lang in preferred_ui_languages() {
            println!("- {lang}");
        }
    }

    #[cfg(feature = "global-pref")]
    println!(
        "GlobalizationPreferences: {}",
        win32_locale_info::globalization_preference()?
    );

    Ok(())
}
