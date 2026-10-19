fn running_sum(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut acc = 0;
    for &v in values {
        acc += v;
        result.push(acc);
    }
    result
}

fn remove_duplicates(values: &mut Vec<i32>) {
    let mut result = Vec::new();
    for &v in values.iter() {
        if !result.contains(&v) {
            result.push(v);
        }
    }
    *values = result;
}

fn double_in_place(values: &mut [i32]) {
    for x in values.iter_mut() {
        *x *= 2;
    }
}

fn largest(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn task4_vectors() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    double_in_place(&mut v);
    
    println!("v = {:?}", v);
    println!("running_sum(&v) = {:?}", running_sum(&v));
    println!("largest(&v) = {:?}", largest(&v));
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(top_left: Point, width: f64, height: f64) -> Self {
        Self { top_left, width, height }
    }

    fn square(top_left: Point, base: f64) -> Self {
        Self { top_left, width: base, height: base }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x && p.x <= self.top_left.x + self.width &&
        p.y >= self.top_left.y && p.y <= self.top_left.y + self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn task5_structs() {
    let mut r1 = Rectangle::new(Point::new(0.0, 0.0), 10.0, 5.0);
    let sq = Rectangle::square(Point::new(1.0, 1.0), 3.0);
    
    println!("r1 area: {}", r1.area());
    println!("r1 contains (1,1): {}", r1.contains(Point::new(1.0, 1.0)));
    println!("r1 can hold sq: {}", r1.can_hold(&sq));
    
    r1.scale(2.0);
    println!("Scaled r1: {:?}", r1);
}

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    fn titles(&self) -> Vec<&str> {
        self.books.iter().map(|b| b.title.as_str()).collect()
    }

    fn by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    fn oldest(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|b| b.year)
    }

    fn remove(&mut self, title: &str) -> Option<Book> {
        if let Some(index) = self.books.iter().position(|b| b.title == title) {
            Some(self.books.remove(index))
        } else {
            None
        }
    }
}

fn task6_library() {
    let mut lib = Library::new();
    lib.add(Book::new("The Rust Programming Language", "Steve Klabnik", 2018));
    lib.add(Book::new("1984", "George Orwell", 1949));
    lib.add(Book::new("Animal Farm", "George Orwell", 1945));

    println!("Titles: {:?}", lib.titles());
    println!("Books by George Orwell: {:?}", lib.by_author("George Orwell"));
    println!("Oldest book: {:?}", lib.oldest());
    
    println!("Removed book: {:?}", lib.remove("1984"));
    println!("Titles after removal: {:?}", lib.titles());
}

fn main() {
    println!("--- Task 4 Tests ---");
    task4_vectors();

    println!("\n--- Task 5 Tests ---");
    task5_structs();

    println!("\n--- Task 6 Tests ---");
    task6_library();
}