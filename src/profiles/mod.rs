mod linux_18;
mod linux_19;
mod linux_20;
mod linux_21_0;
mod linux_21_1;
mod macos_18;
mod macos_19;
mod macos_20;
mod macos_20_3;
mod macos_21_0;
mod macos_21_1;
mod windows_18;
mod windows_18_6;
mod windows_19;
mod windows_19_1;
mod windows_20;
mod windows_21_0;
mod windows_21_1;

use crate::profile::Profile;

pub static ALL: &[Profile] = &[
    linux_18::PROFILE,
    linux_19::PROFILE,
    linux_20::PROFILE,
    linux_21_0::PROFILE,
    linux_21_1::PROFILE,
    windows_18::PROFILE,
    windows_18_6::PROFILE,
    windows_19::PROFILE,
    windows_19_1::PROFILE,
    windows_20::PROFILE,
    windows_21_0::PROFILE,
    windows_21_1::PROFILE,
    macos_18::X86_64,
    macos_18::ARM64,
    macos_19::X86_64,
    macos_19::ARM64,
    macos_20::X86_64,
    macos_20::ARM64,
    macos_20_3::X86_64,
    macos_20_3::ARM64,
    macos_21_0::X86_64,
    macos_21_0::ARM64,
    macos_21_1::X86_64,
    macos_21_1::ARM64,
];
