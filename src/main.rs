#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
    Result,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new();

    let _webview =
        WebViewBuilder::new(WindowBuilder::new().with_title("Revolt").build(&event_loop)?)?
            .with_url("https://discord.com/app")?
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
