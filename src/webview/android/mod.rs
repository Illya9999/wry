use super::{WebContext, WebViewAttributes};
use crate::{application::window::Window, Result};
use std::rc::Rc;
use tao::platform::android::ndk_glue::*;

pub struct InnerWebView {
  pub window: Rc<Window>,
}

impl InnerWebView {
  pub fn new(
    window: Rc<Window>,
    attributes: WebViewAttributes,
    _web_context: Option<&mut WebContext>,
  ) -> Result<Self> {
    let WebViewAttributes {
      url,
      initialization_scripts,
      ipc_handler,
      devtools,
      ..
    } = attributes;

    if let Some(u) = url {
      let mut url_string = String::from(u.as_str());
      let name = u.scheme();
      let schemes = attributes
        .custom_protocols
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
      if schemes.contains(&name) {
        url_string = u
          .as_str()
          .replace(&format!("{}://", name), "https://tauri.mobile/")
      }
      MainPipe::send(WebViewMessage::CreateWebView(
        url_string,
        initialization_scripts,
        devtools,
      ));
    }

    let w = window.clone();
    if let Some(i) = ipc_handler {
      IPC.get_or_init(move || UnsafeIpc::new(Box::into_raw(Box::new(i)) as *mut _, w));
    }

    Ok(Self { window })
  }

  pub fn print(&self) {}

  pub fn eval(&self, _js: &str) -> Result<()> {
    Ok(())
  }

  pub fn focus(&self) {}

  #[cfg(any(debug_assertions, feature = "devtools"))]
  pub fn open_devtools(&self) {}

  #[cfg(any(debug_assertions, feature = "devtools"))]
  pub fn close_devtools(&self) {}

  #[cfg(any(debug_assertions, feature = "devtools"))]
  pub fn is_devtools_open(&self) -> bool {
    false
  }

  pub fn zoom(&self, _scale_factor: f64) {}
}

pub fn platform_webview_version() -> Result<String> {
  todo!()
}
