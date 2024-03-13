fn main() {
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let s = ["Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    for i in (0..12) {
        println!("On the {} day of Christmas,", ordinal[i]);
        println!("my true love gave to me");
        if i == 0 {
            println!("A partridge in a pear tree.\n");
            continue;
        }
        for j in (0..i).rev() {
            println!("{}", s[j]);
        }
        if i == 11 {
            println!("And a partridge in a pear tree!");
            break;
        }
        println!("And a partridge in a pear tree.\n");
    } 
}