fn main() {

    let a : usize = 12;

    println!(a);


    let u = User{
        username: "ss",
        email: "sd",
        sign_in_count: 45,
        active: false,
    };


}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}