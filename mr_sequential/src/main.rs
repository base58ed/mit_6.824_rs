use std::env;
use std::error::Error;
use libloading::Library;
use mr_model::KeyValue;

type MapFn<'m> = libloading::Symbol<'m, fn(String, String) -> Vec<KeyValue>>;
type ReduceFn<'r> = libloading::Symbol<'r, fn(String, Vec<String>) -> String>;


unsafe fn load_dylib(path: &str) -> Library {
    return Library::new(path).expect(format!("couldn't load the dylib: {}", &path).as_str());
}

// cargo run --bin mr_sequential target/debug/libmap_reduce_task.dylib

fn main() -> Result<(), Box<dyn Error>> {
    common::logger::init()?;
    tracing::info!("map/reduce sequential");
    let args = env::args().collect::<Vec<_>>();
    let dlib_path = match args.get(1) {
        Some(path) => path.as_str(),
        None => panic!("dylib path not provided"),
    };

    let lib = unsafe { load_dylib(dlib_path) };
    let map_fn: MapFn = unsafe { lib.get(b"map").expect("map fn not found") };
    let reduce_fn: ReduceFn = unsafe { lib.get(b"reduce").expect("reduce fn not found") };

    tracing::info!("{:?}", map_fn("filename".to_string(), "content".to_string()));
    tracing::info!("{:?}", reduce_fn("key".to_string(), vec!["value".to_string()]));

    Ok(())
}
