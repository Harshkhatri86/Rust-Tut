pub fn lifetime() {
    let ans;
    let str1 = String::from("nin");

    {
        let str2 = String::from("nin");

        ans = longest(&str1, &str2);
        println!("longest String{}", ans)
    }
    // life of a str2 ends with the scope of the block
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}
