#![doc = include_str!("../README.md")]
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
/// - RFC 1766 locale name on success
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
/// You may want to pass [`LOCALE_SISO639LANGNAME`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Globalization/constant.LOCALE_SISO639LANGNAME.html)
/// or [`LOCALE_SISO3166CTRYNAME`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Globalization/constant.LOCALE_SISO3166CTRYNAME.html),
///
/// for ISO 639-1 language code and ISO 3166 region code.
///
/// Returns:
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

/*
   MIT License

   Copyright (c) 2021 1Password

   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:

   The above copyright notice and this permission notice shall be included in all
   copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

/// Available since Windows Vista
///
/// List UI languages in preferred order, each entry is in RFC 1766 format.
#[cfg(feature = "legacy-methods")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "legacy-methods")))]
pub fn preferred_ui_languages() -> impl Iterator<Item = String> {
    use windows::Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME};
    use windows::core::PWSTR;

    let mut num_languages: u32 = 0;
    let mut buffer_length: u32 = 0;

    // Calling this with null buffer will retrieve the required buffer length
    let success = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            None,
            &mut buffer_length,
        )
    }
    .is_ok();
    if !success {
        return Vec::new().into_iter();
    }

    let mut buffer = Vec::<u16>::with_capacity(buffer_length as usize);

    // Now that we have an appropriate buffer, we can query the names
    let mut result = Vec::with_capacity(num_languages as usize);
    let success = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            Some(PWSTR(buffer.as_mut_ptr())),
            &mut buffer_length,
        )
    }
    .is_ok();

    if success {
        // SAFETY: Windows wrote the required length worth of UTF-16 into our buffer, which initialized it.
        unsafe { buffer.set_len(buffer_length as usize) };
        // The buffer contains names split by null char (0), and ends with two null chars (00)
        for part in buffer.split(|i| *i == 0).filter(|p| !p.is_empty()) {
            if let Ok(locale) = String::from_utf16(part) {
                result.push(locale);
            }
        }
    }

    result.into_iter()
}
