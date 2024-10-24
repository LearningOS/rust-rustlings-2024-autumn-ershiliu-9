struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut y: Option<Point> = Some(Point { x: 100, y: 200 });

    // 使用 `ref` 关键字来借用 `y` 而不是移动它
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"),
    }

    // 现在 `y` 仍然可用，因为它没有被移动
    y; 
}