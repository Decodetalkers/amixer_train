use alsa::mixer::{Mixer, SelemChannelId, SelemId};

fn get_volumn(volume: i64) -> i64 {
    volume * 65536 / 100
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the default mixer
    let mixer = Mixer::new("default", false)?;

    // Find the master control
    let sid = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&sid).ok_or("Master control not found")?;

    // Define the left and right channel IDs
    let left = SelemChannelId::FrontLeft;
    let right = SelemChannelId::FrontRight;

    // Set the volume for left and right channels (0 to 100)
    let left_volume = get_volumn(40); // Adjust this value as needed
    let right_volume = get_volumn(70); // Adjust this value as needed
    selem.set_playback_volume(left, left_volume)?;
    selem.set_playback_volume(right, right_volume)?;

    // Optionally, print out the current volumes for verification
    let left_vol = selem.get_playback_volume(left)?;
    let right_vol = selem.get_playback_volume(right)?;
    println!("Left channel volume: {}", left_vol);
    println!("Right channel volume: {}", right_vol);

    Ok(())
}
