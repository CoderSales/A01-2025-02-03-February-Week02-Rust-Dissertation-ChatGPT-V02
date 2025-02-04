# Search

## Search

____

[match in rust - Google Search](https://www.google.com/search?q=match+in+rust&oq=match+in+rust&gs_lcrp=EgZjaHJvbWUqBwgAEAAYgAQyBwgAEAAYgAQyBwgBEAAYgAQyCAgCEAAYFhgeMggIAxAAGBYYHjIICAQQABgWGB4yCAgFEAAYFhgeMggIBhAAGBYYHjIICAcQABgWGB4yCAgIEAAYFhgeMggICRAAGBYYHtIBCDE5NjhqMGo3qAIAsAIA&sourceid=chrome&ie=UTF-8)

[match - Rust By Example](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

____

## Result

Rust provides pattern matching via the match keyword, which can be used like a C switch.

```rust
match number {
// Match a single value
1 => println!("One!"),
// Match several values
2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
```

## Search

____

[match in rust error - Google Search](https://www.google.com/search?q=match+in+rust+error&num=10&newwindow=1&sca_esv=1e35dc1f4110e681&sxsrf=AHTn8zrbUQ3tJfHOLmE9ePWrgGH2b6bakg%3A1738631009954&ei=YWehZ6LuObKjhbIPoNHE0A0&ved=0ahUKEwjiovmd6aiLAxWyUUEAHaAoEdoQ4dUDCBE&uact=5&oq=match+in+rust+error&gs_lp=Egxnd3Mtd2l6LXNlcnAiE21hdGNoIGluIHJ1c3QgZXJyb3IyBhAAGBYYHjILEAAYgAQYhgMYigUyCxAAGIAEGIYDGIoFMgsQABiABBiGAxiKBTILEAAYgAQYhgMYigVIsQ5QygNY-wpwAXgBkAEAmAFEoAHkAqoBATa4AQPIAQD4AQGYAgegAvsCwgIKEAAYsAMY1gQYR8ICDRAAGIAEGLADGEMYigXCAhAQLhiABBiwAxhDGNQCGIoFmAMAiAYBkAYKkgcBN6AH1Rg&sclient=gws-wiz-serp)

[Ok() and Error() and match statements](https://users.rust-lang.org/t/ok-and-error-and-match-statements/45142)

____

## Result

```rust
use std::io;

fn main()
{
    let mut input = String::new();

    match io::stdin().read_line(&mut input)
    {
        Ok(_) => println!("input: {}", input),
        Err(_) => println!("Error!"),
    }
}
```
