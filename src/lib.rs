// PylonTech based on SimpBMS
// Data references  https://github.com/Tom-evnut/BMWI3BMS/blob/master/BMWI3BMS.ino

use log::info;
use solax_can_bus::SolaxBms;
use std::time::Duration;

pub fn custom_frames(
    bmsdata: SolaxBms,
    charger_volts_high: Option<u16>,
    charger_volts_low: Option<u16>,
    timeout: Duration,
) -> anyhow::Result<Vec<(u16, Vec<u8>)>> {
    match bmsdata.timestamp {
        Some(time) => {
            if time.elapsed() < timeout {
                info!("Data is {:?} old", time.elapsed(),);
            } else {
                return Err(anyhow::anyhow!(
                    "Data is too old {:?}, timeout is {:?}",
                    time.elapsed(),
                    timeout
                ));
            }
        }
        None => return Err(anyhow::anyhow!("BMS timestamp is invalid")),
    }
    let charge_volts_high = match charger_volts_high {
        Some(val) => val.to_le_bytes(),
        None => 3936u16.to_le_bytes(), // 96 * 4.1v
    };

    let charge_volts_low = match charger_volts_low {
        Some(val) => val.to_le_bytes(),
        None => 2880u16.to_le_bytes(), // 96 * 3v
    };
    let charge_current = bmsdata.charge_max.to_le_bytes();
    let discharge_current = bmsdata.discharge_max.to_le_bytes();
    let soc = bmsdata.capacity.to_le_bytes();
    let soh = 108u16.to_le_bytes();
    let pack_volts = bmsdata.voltage.to_le_bytes();
    let current = bmsdata.current.to_le_bytes();
    let temp = bmsdata.int_temp.to_le_bytes();

    let cap = (52000u16 / 65).to_le_bytes(); // capacity vs voltage
    let temp_high = bmsdata.cell_temp_max.to_le_bytes();
    let temp_low = bmsdata.cell_temp_min.to_le_bytes();

    Ok([
        (0x359, [0x0, 0x0, 0x0, 0x0, 0x22, 0x50, 0x4e, 0x0].to_vec()),
        (
            0x351,
            [
                charge_volts_high[0],
                charge_volts_high[1],
                charge_current[0],
                charge_current[1],
                discharge_current[0],
                discharge_current[1],
                0x0,
                0x0,
            ]
            .to_vec(),
        ),
        (
            0x355,
            [soc[0], soc[1], soh[0], soh[1], 0x0, 0x0, 0x0, 0x0].to_vec(),
        ),
        (
            0x356,
            [
                pack_volts[0],
                pack_volts[1],
                current[0],
                current[1],
                temp[0],
                temp[1],
            ]
            .to_vec(),
        ),
        (0x35c, [0xc0, 0x00].to_vec()),
        (0x35e, [b'T', b'P'].to_vec()),
    ]
    .to_vec())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
