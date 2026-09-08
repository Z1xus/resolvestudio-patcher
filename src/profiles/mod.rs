mod linux_21_1;
mod windows_21_1;

use crate::profile::Profile;

pub static ALL: &[Profile] = &[linux_21_1::PROFILE, windows_21_1::PROFILE];
