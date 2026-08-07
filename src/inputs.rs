use std::sync::{LazyLock, RwLock};

use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::KEY_COUNT;

/* 
    Stores the last out-of-range input code so device.rs can check it
    after an error, since process_input can't trigger events directly.
*/
pub static LAST_BAD_INPUT: LazyLock<RwLock<Option<u8>>> = LazyLock::new(|| RwLock::new(None));

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing input: {}, {}", input, state);

    match input as usize {
        0..=KEY_COUNT => read_button_press(input, state),
        // 50 is the "bad" input code that indicates the device has been reset (e.g. screensaver exit)
        _ => {
            *LAST_BAD_INPUT.write().unwrap() = Some(input);
            Err(MirajazzError::BadData)
        }
    }
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

/*  Converts between OpenDeck's 0-indexed key numbering and the device's 1-indexed numbering.
    OpenDeck: 0-11
    Device: 1-12
*/
pub fn opendeck_to_device(key: u8) -> u8 {
    key
}

pub fn device_to_opendeck(key: usize) -> usize {
    key - 1
}

fn read_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    let pressed_index: usize = device_to_opendeck(input as usize);
    button_states[pressed_index + 1] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}