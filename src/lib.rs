#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

use std::time::Instant;

#[lua_function]
unsafe fn hook_call_test(lua: gmod::lua::State) -> i32 {
    let start = Instant::now();

    lua.get_global(lua_string!("hook"));
    for _ in 0..=99999 {
        lua.get_field(-1, lua_string!("Call"));
        lua.push_string("HOOK_CALL_TEST");
        lua.push_nil();
        lua.push_string("test");
        lua.call(3, 0);
    }
    lua.pop();

    let duration = start.elapsed();
    let seconds = duration.as_secs_f64();

    lua.push_number(seconds);

    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    lua.push_function(hook_call_test);
    lua.set_global(lua_string!("HOOK_CALL_TEST"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    println!("Goodbye xd!");
    0
}
