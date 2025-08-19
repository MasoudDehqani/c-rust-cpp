// A unit struct (useful for generics)
struct Unit;

// A tuple struct (which is in fact a named tuple)
struct Pair(i32, f32);

#[derive(Debug)]
struct User {
    username: String,
    is_active: bool,
    login_count: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn make_with_height(height: u32) -> Rectangle {
        Rectangle { width: 20, height }
    }
}

fn main() {
    let pair = Pair(-23, 10.3);
    println!("{}, {}", pair.0, pair.1);

    let _unit = Unit;

    let user: User = User {
        username: String::from("username"),
        is_active: true,
        login_count: 1,
    };

    // destructuring a struct and using the .. wildcard to ignore the remaining field
    let User { username, .. } = &user;
    let User {
        is_active,
        login_count,
        ..
    } = &user;

    println!("{:#?}", user);
    println!("{user:#?}");
    println!("{username}, {is_active}, {login_count}");

    let rect: Rectangle = Rectangle {
        width: 34,
        height: 45,
    };

    let area = area_sep(rect.width, rect.height);
    println!("{area}");
    let area = area_tup((rect.width, rect.height));
    println!("{area}");
    let area = area_rect(&rect);

    println!("{:?}", rect);
    println!("{area}");

    let area = rect.area();
    println!("{area}");

    let new_rect = Rectangle::new(12, 90);
    println!("{:?}", new_rect);

    let another_rect = Rectangle::make_with_height(40);
    println!("{:?}", another_rect);

    let can_embrace = rect.can_hold(&another_rect);
    println!("{}", can_embrace);
}

fn area_sep(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimenstions: (u32, u32)) -> u32 {
    dimenstions.0 * dimenstions.1
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
