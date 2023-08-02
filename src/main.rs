#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use open::that;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;
use wry::Result;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();

    let _webview = WebViewBuilder::new(
        WindowBuilder::new()
            .with_title("Revolt")
            .build(&event_loop)?,
    )?
    .with_url("https://discord.com/app")?
    .with_new_window_req_handler(|url| {
        let _ = that(url);
        false
    })
    .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
