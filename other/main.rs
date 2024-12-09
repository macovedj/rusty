// #[allow(warnings)]
// mod bindings;

// use bindings::exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};

// struct Component;

// impl Guest for Component {
//     fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
//         unimplemented!()
//     }
// }

fn main() {
    // let now = core::time::SystemTime::now();
    // let elapsed = now.elapsed();
    // dbg!(&elapsed);
    let foo = [::core::mem::MaybeUninit::uninit(); 1];

    // println!("{}", now);
}