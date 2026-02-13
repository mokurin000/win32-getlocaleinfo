#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "global-pref")]
use std::error::Error;
#[cfg(feature = "legacy-methods")]
use std::ffi::OsString;
#[cfg(feature = "legacy-methods")]
use std::os::windows::ffi::OsStringExt;

#[cfg(feature = "legacy-methods")]
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;

/// Available since Windows Vista
///
/// returns:
/// - empty string on failure
/// - rfc 1766 locale name on success
#[cfg(feature = "legacy-methods")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "legacy-methods")))]
pub fn get_locale_lcid() -> String {
    use windows::Win32::Globalization::{LCIDToLocaleName, LOCALE_USER_DEFAULT};

    let mut buf = vec![0_u16; LOCALE_NAME_MAX_LENGTH as _];

    debug_assert!(buf.len() != 0);

    let return_code = unsafe { LCIDToLocaleName(LOCALE_USER_DEFAULT, Some(&mut buf), 0) };
    buf.truncate(return_code as _);

    OsString::from_wide(&buf)
        .to_string_lossy()
        .trim()
        .to_string()
}

/// Available since Windows 2000
///
/// returns:
/// - empty string on failure
/// - expected lctype result on success
#[cfg(feature = "legacy-methods")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "legacy-methods")))]
pub fn get_locale_info(lctype: u32) -> String {
    let mut buf = vec![0_u16; LOCALE_NAME_MAX_LENGTH as _];
    let return_code = unsafe {
        use windows::Win32::Globalization::{GetLocaleInfoW, LOCALE_USER_DEFAULT};
        GetLocaleInfoW(LOCALE_USER_DEFAULT, lctype, Some(&mut buf))
    };
    buf.truncate(return_code as _);

    OsString::from_wide(&buf)
        .to_string_lossy()
        .trim()
        .to_string()
}

/// Available since Windows Vista
///
/// returns:
/// - empty string on failure
/// - RFC 1766 locale name on success
#[cfg(feature = "legacy-methods")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "legacy-methods")))]
pub fn get_user_default_locale_name() -> String {
    let mut buf = vec![0_u16; LOCALE_NAME_MAX_LENGTH as _];
    let return_code = unsafe {
        use windows::Win32::Globalization::GetUserDefaultLocaleName;
        GetUserDefaultLocaleName(&mut buf)
    };
    buf.truncate(return_code as _);
    OsString::from_wide(&buf)
        .to_string_lossy()
        .trim()
        .to_string()
}

/// Available since Windows 10.0.10240.0
///
/// returns BCP-47 language code on success.
#[cfg(feature = "global-pref")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "global-pref")))]
pub fn globalization_preference() -> Result<String, Box<dyn Error + Send + Sync>> {
    use windows::System::UserProfile::GlobalizationPreferences;

    Ok(GlobalizationPreferences::Languages()?
        .First()?
        .Current()?
        .to_string())
}
