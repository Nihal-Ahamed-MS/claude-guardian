#[cfg(not(target_os = "linux"))]
mod imp {
    use anyhow::Result;
    use tokio::runtime::Runtime;
    use tray_icon::{
        menu::{Menu, MenuEvent, MenuItem},
        TrayIcon, TrayIconBuilder, TrayIconEvent,
    };
    use winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        window::WindowId,
    };

    struct TrayApp {
        rt: Option<Runtime>,
        tray: Option<TrayIcon>,
        open_id: Option<tray_icon::menu::MenuId>,
        exit_id: Option<tray_icon::menu::MenuId>,
    }

    impl ApplicationHandler for TrayApp {
        fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
            if self.tray.is_some() {
                return;
            }

            let menu = Menu::new();
            let open_item = MenuItem::new("View logs", true, None);
            let exit_item = MenuItem::new("Exit", true, None);
            self.open_id = Some(open_item.id().clone());
            self.exit_id = Some(exit_item.id().clone());
            let _ = menu.append(&open_item);
            let _ = menu.append(&exit_item);

            // 1×1 transparent icon — replace with a real asset later.
            let icon = tray_icon::Icon::from_rgba(vec![0u8, 0u8, 0u8, 0u8], 1, 1)
                .expect("failed to create tray icon");

            self.tray = TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_tooltip("llm-observer")
                .with_icon(icon)
                .build()
                .ok();
        }

        fn window_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _id: WindowId,
            _event: WindowEvent,
        ) {
        }

        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            while TrayIconEvent::receiver().try_recv().is_ok() {}

            while let Ok(ev) = MenuEvent::receiver().try_recv() {
                if Some(&ev.id) == self.open_id.as_ref() {
                    let _ = open::that("http://localhost:7422");
                } else if Some(&ev.id) == self.exit_id.as_ref() {
                    drop(self.tray.take());
                    if let Some(rt) = self.rt.take() {
                        rt.shutdown_background();
                    }
                    event_loop.exit();
                }
            }

            event_loop.set_control_flow(ControlFlow::Wait);
        }
    }

    pub fn run_event_loop(rt: Runtime) -> Result<()> {
        let event_loop = EventLoop::new()?;

        let mut app = TrayApp {
            rt: Some(rt),
            tray: None,
            open_id: None,
            exit_id: None,
        };

        event_loop.run_app(&mut app)?;
        Ok(())
    }
}

pub use imp::run_event_loop;
