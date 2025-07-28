use serde::Serialize;

#[derive(Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("Serialized point = {serialized}",);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
