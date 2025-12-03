pub fn recite(start_bottles: u32, take_down: u32) -> String {
    const NUMBERS: [&str; 11] = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    let mut bottles_remaining = start_bottles;
    let mut full_song = String::new();

    fn bottle_form(n: u32) -> &'static str {
        if n == 1 { "bottle" } else { "bottles" }
    }
    while bottles_remaining > (start_bottles - take_down) {
        let mut container = bottle_form(bottles_remaining);

        full_song.push_str(&format!(
            "{} green {} hanging on the wall,\n",
            NUMBERS[bottles_remaining as usize], container
        ));

        full_song.push_str(&format!(
            "{} green {} hanging on the wall,\n",
            NUMBERS[bottles_remaining as usize], container
        ));

        full_song.push_str("And if one green bottle should accidentally fall,\n");

        bottles_remaining -= 1;

        container = bottle_form(bottles_remaining);

        full_song.push_str(&format!(
            "There'll be {} green {} hanging on the wall.\n",
            NUMBERS[bottles_remaining as usize].to_lowercase(),
            container
        ));

        if bottles_remaining > (start_bottles - take_down) {
            full_song.push('\n')
        }
    }
    full_song
}
