pub fn change_player(player: &mut char) {
    let p = player;

    match *p {
        'x' => *p = 'o',
        'o' => *p = 'x',
        _ => panic!("Unexpected player character {}.", *p)
    }
}