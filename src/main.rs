

fn main() {
    println!("\n\n\tStructs\n\n");

    struct_immutable();
    struct_mutable();
    return_instance();
    different_types_with_tuple_struct();
    // defining_unit_like_struct();
    ownership_of_struct_data();

    // ----------
    rectangles_attempt_1();
    rectangles_attempt_2();
    rectangles_attempt_3();
    rectangle_method();
    associated_functions();

}

fn associated_functions() {
    println!("\n\n\tAssociated Functions\n");

    let square = Rectangle::square(10);
    println!(
        "The area of the rectangle is {} square pixels.",
        square.area());
    println!("---------------------------");
    println!("rectangle height : {}", square.height);
    println!("rectangle width : {}", square.width);
    println!("\nsquare is : {square:#?}");

}

fn rectangle_method() {
    println!("\n\n\tRectangles Attempt 4");
    println!("\t\tStruct Methods\n");

    let rec1 = Rectangle {
        width : 30,
        height : 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area());
    println!("---------------------------");
    println!("rectangle height : {}", rec1.height);
    println!("rectangle width : {}", rec1.width);
    println!("\nrec1 is : {rec1:#?}");
    if rec1.width() {
        println!("The rectangle has a nonzero width; it is {}", rec1.width);
    }


    println!("\nmethod with multiple arguments");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

impl Rectangle {
    // Associated Functions
    fn square(size: u32) -> Self {
        Self {
            width : size,
            height : size,
        }
    }
}

fn rectangles_attempt_3() {
    println!("\n\n\tRectangles Attempt 3\n");
    let rec1 = Rectangle{
        width : 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rec1)
    );
    //  we’ve named rectangle, whose type is an immutable
    // borrow of a struct Rectangle instance. As mentioned
    // in Chapter 4, we want to borrow the struct rather
    // than take ownership of it. This way, main retains
    // its ownership and can continue using rect1, which
    // is the reason we use the & in the function signature
    // and where we call the function.
    println!("rectangle height : {}", rec1.height);
    println!("rectangle width : {}", rec1.width);

    // let statement = '''
    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }''';

    // This won’t work, however.
    println!("\n----------------------------------");
    println!("\nDebug printing");
    println!("\nrec1 is ex-1 {rec1:?}");
    println!("\nrec1 is ex-2 {rec1:#?}");
    println!("\nrec1 is ex-3 ");
    dbg!(&rec1);

    {
        println!("\nrec1 is ex-4 ");
        let scale = 2;
        let rec2 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        println!("dbg!(&rec2);");
        dbg!(&rec2);
    }

}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn rectangles_attempt_2() {
    println!("\n\n\tRectangles Attempt 2\n");
    let rec1 = (30, 50);

    println!(
        "The area2 of the rectangle is {} square pixels.",
        area2(rec1)
    );

}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area1(width: u32, height: u32) -> u32 {

    width * height
}
fn rectangles_attempt_1() {
    println!("\n\n\tRectangles Attempt 1\n");

    let width1 = 30;
    let height1 = 50;

    println!("The area1 of the rectangle is {} square pixels.",
        area1(width1, height1));
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
//         username: "some username123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
}

// struct AlwaysEqual;
//
// fn defining_unit_like_struct() {
//     println!("Defining Unit-Like Structs");
//
//     let subject = AlwaysEqual;
//
//
// }

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
    //     email: String::from("another@example.com"),
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
        username: String::from("some username123"),
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
        username: String::from("some username123"),
        email:String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email address: {}", user1.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    pub sign_in_count: u64,
}


