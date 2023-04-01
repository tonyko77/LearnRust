//----------------------------------------------------------------
// Structs

pub fn test_structs() {
    println!("\n=============================\n--> Structs:");

    let mut user = User::create("tonyko");
    user.println();

    user.update_email("me@nowhere.com");
    user.sign_in();
    user.deactivate();
    user.println();

    user.activate();
    user.sign_in();
    user.println();

    let other = user.copy("gogu");
    other.println();

    // debug printing
    println!("{:?}", other);

    // default initialization of User
    let default_user = User::default();
    println!("-> defaultUser = {:?}", default_user);

    let color = Color::new(12, 34, 56);
    color.println();    
}


// A regular struct
#[derive(Debug, Default)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    pub fn create(name: &str) -> User {
        User {
            username: String::from(name),
            email: String::from(""),
            active: true,
            sign_in_count: 0,
        }
    }

    pub fn copy(&self, name: &str) -> User {
        // using the struct update syntax here :)
        User {
            username: String::from(name),
            email: String::from(""),
            ..*self
        }
    }

    pub fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    pub fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    fn println(&self) {
        println!("User({} <{}> - {}, sign in count: {})",
            self.username,
            self.email,
            if self.active { "active" } else { "inactive"},
            self.sign_in_count);
    }
}


// A tuple struct
struct Color(i32, i32, i32);

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color(r, g, b)
    }

    pub fn r(&self) -> i32 {
        self.0
    }

    pub fn g(&self) -> i32 {
        self.1
    }

    pub fn b(&self) -> i32 {
        self.2
    }

    fn println(&self) {
        println!("Color(r:{}, g:{}, b:{})", self.0, self.1, self.2);
    }
}
