#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        unimplemented!()
    }
}

bindings::export!(Component with_types_in bindings);
