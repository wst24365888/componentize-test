mod bindings;
use bindings::exports::local::hello::greeting::Guest;
use bindings::wasi::foo::bar::print_hello;

struct Component;

impl Guest for Component {
    fn say_hello(name: String) -> String {
        format!("{} {}", print_hello(), name)
    }
}

bindings::export!(Component with_types_in bindings);
