fn main() {
    let mut cereals: [String; 5] = [String::from("Cookie Crisp"), String::from("Cinnamon Toast Crunch"), String::from("Frosted Flakes"), String::from("Cocoa Puffs"), String::from("Captain Crunch")];

    let first_two: &[String] = &cereals[..2];
    println!("{first_two:?}");

    let mid_three: &[String] = &cereals[1..4];
    println!("{mid_three:?}");

    let last_three: &mut [String] = &mut cereals[2..];
    println!("{last_three:?}");

    last_three[2] = String::from("Lucky Charms");
    println!("{last_three:?}");

    println!("{cereals:?}");

    let cookie_crisp: &String = &cereals[0];
    println!("{cookie_crisp:?}");

    let cookie: &str = &cookie_crisp[..6];
    println!("{cookie}");

    let cocoa_puffs: &String = &cereals[3];
    println!("{cocoa_puffs:?}");

    let puffs: &str = &cocoa_puffs[6..];
    println!("{puffs:?}")
}
