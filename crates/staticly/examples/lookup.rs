use staticly::{hash, static_enum, static_map, static_set, switch, algo::fnv1a};

static_map! {
    HTTP_STATUS: u64 = {
        "OK" => 200,
        "Created" => 201,
        "No Content" => 204,
        "Bad Request" => 400,
        "Not Found" => 404,
        "Internal Server Error" => 500,
    };
}

static_set! {
    HTTP_METHODS = [
        "GET",
        "POST",
    ];
}

static_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Color {
        Red = "red",
        Green = "green",
        Blue = "blue",
    }
}

fn main() {
    const GET_HASH: u64 = hash!("get");
    const GREEN_HASH: u64 = Color::Green.hash();
    const BLUE_STR: &str = Color::Blue.as_str();

    assert_eq!(GET_HASH, fnv1a(b"get"));

    assert!(!HTTP_STATUS.is_empty());
    assert_eq!(HTTP_STATUS.len(), 6);
    assert!(HTTP_STATUS.contains_key("OK"));

    assert_eq!(HTTP_STATUS.get("OK"), Some(200));
    assert_eq!(HTTP_STATUS.get_hash(hash!("Not Found")), Some(404));

    println!("status:");

    for (name, status) in HTTP_STATUS.iter() {
        println!("  {name} = {status}");
    }

    assert!(!HTTP_METHODS.is_empty());
    assert_eq!(HTTP_METHODS.len(), 2);
    assert!(HTTP_METHODS.contains("GET"));

    println!("\nmethods:");

    for name in HTTP_METHODS.iter() {
        println!("  {name}");
    }

    let val = switch!("123", {
        "123" => 123,
        _ => 0,
    });
    assert_eq!(val, 123);

    assert_eq!("red".parse(), Ok(Color::Red));
    assert_eq!(BLUE_STR, "blue");
    assert_eq!(GREEN_HASH, hash!("green"));
}
