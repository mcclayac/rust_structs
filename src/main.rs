

fn main() {
    println!("\n\n\tStructs\n\n");

    struct_immutable();
    struct_mutable();

}

fn struct_mutable() {
    println!("\n\n\tStruct Mutable\n\n");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("email address: {}", user1.email);
}

fn struct_immutable() {
    println!("\n\n\tStruct Immutable\n\n");

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email address: {}", user1.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


