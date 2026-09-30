use staticly::{hash, static_map, static_set, switch};

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

fn main() {
    let hashed_ident = hash!(get);
    let hashed_str = hash!("get");
    let hashed_bytes = hash!(b"get");
    let hashed_cstr = hash!(c"get");

    assert_eq!(hashed_str, hashed_ident);
    assert_eq!(hashed_str, hashed_bytes);
    assert_eq!(hashed_str, hashed_cstr);

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
    assert_eq!(HTTP_METHODS.len(), 1);
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
}
