#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
use open::that as open;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{WebContext, WebViewBuilder};
use wry::Result;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();

    let project_dirs = ProjectDirs::from("", "kiracoding", env!("CARGO_PKG_NAME"))
        .map(|project_dirs| project_dirs.config_dir().to_path_buf());

    let web_context = &mut WebContext::new(project_dirs);

    let window = WindowBuilder::new()
        .with_title("Revolt")
        .build(&event_loop)?;

    let init_script = include_str!(concat!(env!("OUT_DIR"), "/init.js"));

    let _webview = WebViewBuilder::new(window)?
        .with_url("https://discord.com/app")?
        .with_web_context(web_context)
        .with_initialization_script(init_script)
        .with_new_window_req_handler(|url| {
            let _ = open(url);
            false
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        };
    });
}
