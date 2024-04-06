## Build the entire workspace modules
cargo build

## Compile the map/reduce task plugin
cargo build -p map_reduce_task

## Run the mr_sequential program to load the map/reduce task and invoke the functions
cargo run --bin mr_sequential target/debug/libmap_reduce_task.dylib
