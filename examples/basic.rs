#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use tyght_map::*;

fn insert_string<S>(map: TyghtMap<S>) -> TyghtMap<S::InsertOutput>
where
    S: Missing<String>,
{
    map.insert("Hello".to_string())
}

fn print_string<S>(map: &TyghtMap<S>)
where
    S: Contains<String>,
{
    let string: &String = map.get();
    println!("{string}");
}

fn main() {
    // Insert some different integer types into the map and check the size
    let map = TyghtMap::new().insert(3u32).insert(4i32).insert(3f32);
    assert_eq!(std::mem::size_of_val(&map), 12);

    // Retrieve the `u32` from the map
    let item: &u32 = map.get();
    assert_eq!(*item, 3);

    // Inserting twice replaces the current value
    let (item, map) = map.try_insert(5u32);
    assert_eq!(std::mem::size_of_val(&map), 12);
    assert_eq!(Some(3u32), item);
    assert_eq!(5u32, *map.get());

    // Insert a string and then print it using generic methods
    let mut map = insert_string(map);
    print_string(&map);

    // Mutate an item
    *map.get_mut::<String>() += ", world!";

    // Remove an item
    let (item, _map) = map.remove::<String>();
    println!("{item}");
}
