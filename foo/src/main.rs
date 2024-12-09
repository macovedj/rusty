use semver::Version;
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
    // let foo: [::core::mem::MaybeUninit<u8>; 1] = [::core::mem::MaybeUninit::uninit(); 1];
    let foo = Version::parse("0.1.0").unwrap();
    dbg!(&foo);

    // println!("{}", now);
}