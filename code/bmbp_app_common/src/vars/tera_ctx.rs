use tera::Context;

use crate::APP_TITLE;

pub fn page_context(func: &str) -> Context {
    let mut ctx = Context::new();
    ctx.insert("appTitle", APP_TITLE);
    let script_vec = vec![
        format!("{}/index.js", func),
        format!("{}/api.js", func),
        format!("{}/form.js", func),
    ];
    ctx.insert("viewScript", script_vec.as_slice());
    ctx.insert("viewName", "PageView");
    ctx
}
