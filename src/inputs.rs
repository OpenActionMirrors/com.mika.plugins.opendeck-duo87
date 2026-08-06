use std::sync::{LazyLock, RwLock};

use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::KEY_COUNT;

/// Records the raw input code whenever the device sends something outside
/// the normal button range (0..=KEY_COUNT), so device.rs can inspect it
/// after catching the resulting error — e.g. detecting screensaver-exit (50).
pub static LAST_BAD_INPUT: LazyLock<RwLock<Option<u8>>> = LazyLock::new(|| RwLock::new(None));

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing input: {}, {}", input, state);

    match input as usize {
        0..=KEY_COUNT => read_button_press(input, state),
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

/// DUO87's button numbering (1-12) already matches visual grid order,
/// top-left to bottom-right — confirmed via USB capture. No remapping needed.
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