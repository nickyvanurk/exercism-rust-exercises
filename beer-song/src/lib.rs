pub fn verse(n: i32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    } else if n == 2 {
        return String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1));
    }
    
    String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1))
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::from("");

    for x in (end..start + 1).rev() {
        song.push_str(verse(x).as_str());
        if x > end {
            song.push_str("\n");
        }
    }

    song
}
