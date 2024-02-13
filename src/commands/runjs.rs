use std::sync::Once;

use rusty_v8 as v8;
use v8::inspector::{
    StringView, V8Inspector, V8InspectorClientBase, V8InspectorClientImpl, V8StackTrace,
};

use crate::data::{Context, Result};

static INITIALIZE: Once = Once::new();

struct InspectorClient(V8InspectorClientBase);

impl InspectorClient {
    fn new() -> Self {
        Self(V8InspectorClientBase::new::<Self>())
    }
}

impl V8InspectorClientImpl for InspectorClient {
    fn base(&self) -> &V8InspectorClientBase {
        &self.0
    }

    fn base_mut(&mut self) -> &mut V8InspectorClientBase {
        &mut self.0
    }

    fn console_api_message(
        &mut self,
        _context_group_id: i32,
        _level: i32,
        message: &StringView,
        _url: &StringView,
        _line_number: u32,
        _column_number: u32,
        _stack_trace: &mut V8StackTrace,
    ) {
        println!("{}", message);
    }
}

#[poise::command(prefix_command, guild_only, owners_only, broadcast_typing)]
pub async fn runjs(ctx: Context<'_>, #[rest] code: String) -> Result {
    INITIALIZE.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });

    let mut final_data: Vec<String> = Vec::new();

    //let polyfill = include_str!("./consoleLog.js");

    {
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        let mut client = InspectorClient::new();
        let mut inspector = V8Inspector::create(isolate, &mut client);

        let handle_scope = &mut v8::HandleScope::new(isolate);

        let context = v8::Context::new(handle_scope);

        inspector.context_created(context, 1, StringView::empty());

        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let code = v8::String::new(scope, &code).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();

        let result = result.to_string(scope).unwrap();
        final_data.push(result.to_rust_string_lossy(scope));
    }

    ctx.reply(format!("```js\n{}\n```", final_data.join("\n")))
        .await?;

    Ok(())
}
