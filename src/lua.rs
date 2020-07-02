use crate::resources::LuaRes;

pub trait LuaResExt {
    fn run_lua_code(&mut self, code: String);
    fn run_lua_file(&self, filename: impl AsRef<std::path::Path> + std::clone::Clone);
}

impl LuaResExt for LuaRes {
    fn run_lua_code(&mut self, code: String) {
        self.lock().unwrap().context(|lua_ctx| {
            lua_ctx.load(&code).exec().unwrap();
        });
    }

    fn run_lua_file(&self, filename: impl AsRef<std::path::Path> + std::clone::Clone) {
        self.lock().unwrap().context(|lua_ctx| {
            let lua_code = std::fs::read_to_string(filename.clone()).unwrap();
            if let Err(e) = lua_ctx
                .load(&lua_code)
                .set_name(&filename.as_ref().file_name().unwrap().to_str().unwrap())
                .unwrap()
                .exec()
            {
                println!("Lua {}", e.to_string());
            };
        });
    }
}
