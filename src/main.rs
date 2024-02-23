#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use directories::ProjectDirs;
use open::that as open;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::{WebContext, WebViewBuilder};

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/init.js"));

fn main() -> Result<()> {
    let project_dirs = ProjectDirs::from("", AUTHOR, NAME)
        .map(|project_dirs| project_dirs.config_dir().to_path_buf());

    let web_context = &mut WebContext::new(project_dirs);

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().with_title(NAME).build(&event_loop)?;

    let _webview = WebViewBuilder::new(&window)
        .with_url("https://discord.com/app")
        .with_web_context(web_context)
        .with_initialization_script(SCRIPT)
        .with_new_window_req_handler(|url| {
            open(url).unwrap();
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
