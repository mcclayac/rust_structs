

fn main() {
    println!("\n\n\tStructs\n\n");

    struct_immutable();
    struct_mutable();
    return_instance();
    different_types_with_tuple_struct();
    defining_unit_like_struct();
    ownership_of_struct_data();

}

// this won't work without lifetimes
// struct user2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
fn ownership_of_struct_data() {
    println!("\n\nOwnership of Struct Data\n");
//     let user1 = User2 {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
}

struct AlwaysEqual;

fn defining_unit_like_struct() {
    println!("Defining Unit-Like Structs");

    let subject = AlwaysEqual;


}

struct Color(i32, i32, i32);
struct Point(i32,i32,i32);

fn different_types_with_tuple_struct() {
    println!("\n\n\tDifferent Types with tuple structs.\n\n");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!("Point x{}, y{}, z{}", x, y, z);

    let Color(a, b, c) = black;
    println!("Color a {}, b {}, c{}", a, b, c);

}

fn return_instance() {
    println!("\n\n\tReturn Instance more\n\n");

    let mut user1 = build_user(String::from("bigmanuser"),
                               String::from("bigman@example.com"));

    println!("email address: {}", user1.email);
    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("sign in count: {}", user1.sign_in_count);

    user1.sign_in_count += 5;
    println!("sign in count: {}", user1.sign_in_count);

    println!("\nCreating User2 ");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: Strring::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 =  User {
        email: String::from("another2@example.com"),
        ..user1
    };

    println!("user2.username: {}", user2.username);

    println!("--------------");
    println!("Trying to re-print user1, but cannot because username has been used");
    println!("email address: {}", user1.email);
    // println!("username: {}", user1.username);
    // Value used after being moved [E0382]

    println!("active: {}", user1.active);
    println!("sign in count: {}", user1.sign_in_count);


}

fn build_user(email: String, username:String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        username,
        email,
    }
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


