use mlua::{Lua, Result};

pub fn init_lua_ui() -> Result<Lua> {
    let lua = Lua::new();
    
    let ui = lua.create_table()?;
    ui.set("set_title", lua.create_function(|_, title: String| {
        println!("UI Title Set: {}", title);
        Ok(())
    })?)?;

    lua.globals().set("ui", ui)?;

    if let Ok(script) = std::fs::read_to_string("ui.lua") {
        lua.load(&script).exec()?;
        lua.load("setup_ui()").exec()?;
    }

    Ok(lua)
}
