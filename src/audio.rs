// Win32 Audio Subsystem for Sager
// Plays authentic Steam achievement unlock chimes

#[link(name = "winmm")]
extern "system" {
    fn PlaySoundA(pszSound: *const u8, hmod: isize, fdwSound: u32) -> i32;
}

const SND_ASYNC: u32 = 0x0001;
const SND_NODEFAULT: u32 = 0x0002;
const SND_MEMORY: u32 = 0x0004;

pub static STEAM_ACHIEVEMENT_WAV: &[u8] = include_bytes!("../assets/sounds/steam_achievement.wav");

/// Plays the authentic Steam achievement sound asynchronously from memory
pub fn play_steam_achievement_sound() {
    unsafe {
        PlaySoundA(
            STEAM_ACHIEVEMENT_WAV.as_ptr(),
            0,
            SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_sound() {
        assert!(!STEAM_ACHIEVEMENT_WAV.is_empty());
        play_steam_achievement_sound();
        std::thread::sleep(std::time::Duration::from_millis(600));
    }
}
