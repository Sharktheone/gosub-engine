use gosub_interface::chrome::ChromeHandle;
use gosub_interface::config::{HasTreeDrawer, ModuleConfiguration};
use gosub_interface::draw::TreeDrawer;
use gosub_interface::eventloop::EventLoopHandle;
use gosub_interface::instance::{Handles, InstanceId};
use gosub_interface::render_backend::{ImageBuffer, NodeDesc};
use gosub_shared::geo::SizeU32;
use gosub_shared::types::Result;
use std::sync::mpsc::Sender as SyncSender;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;
use tokio::task::{spawn_blocking, LocalSet};
use url::Url;

pub struct EngineInstance<C: ModuleConfiguration> {
    pub title: String,
    pub url: Url,
    pub data: C::TreeDrawer,
    rx: Receiver<InstanceMessage>,
    irx: Receiver<InternalInstanceMessage<C>>,
    el: El<C>,
    id: InstanceId,
    handles: Handles<C>,
}

impl<C: ModuleConfiguration> EngineInstance<C> {
    pub async fn new(
        url: Url,
        layouter: C::Layouter,
        id: InstanceId,
        handles: Handles<C>,
    ) -> Result<(Self, InstanceHandle)> {
        let (tx, rx) = tokio::sync::mpsc::channel(64);

        let instance = EngineInstance::with_chan(url.clone(), layouter, rx, id, handles).await?;

        let handle = InstanceHandle { tx };

        Ok((instance, handle))
    }

    pub async fn with_chan(
        url: Url,
        layouter: C::Layouter,
        rx: Receiver<InstanceMessage>,
        id: InstanceId,
        handles: Handles<C>,
    ) -> Result<Self> {
        let data = C::TreeDrawer::from_url(url.clone(), layouter, false).await?;

        let (itx, irx) = tokio::sync::mpsc::channel(16);

        Ok(EngineInstance {
            title: "Gosub".to_string(),
            url,
            data,
            rx,
            el: El(itx),
            irx,
            id,
            handles,
        })
    }

    pub fn new_on_thread(url: Url, layouter: C::Layouter, id: InstanceId, handles: Handles<C>) -> Result<InstanceHandle>
    where
        C::Layouter: Send + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::channel(64);

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();

            let mut instance = match rt.block_on(Self::with_chan(url, layouter, rx, id, handles)) {
                Ok(instance) => instance,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            instance.run(&rt);
        });

        Ok(InstanceHandle { tx })
    }

    fn run(&mut self, rt: &Runtime) {
        let set = LocalSet::new();

        set.block_on(rt, async move {
            while let Some(message) = self.rx.recv().await {
                self.handle_message(message).await;
            }
        });
    }

    async fn handle_message(&mut self, message: InstanceMessage) {
        match message {
            InstanceMessage::Redraw(size) => {
                let scene = self.data.draw(size, &self.el);

                self.handles.chrome.draw_scene(scene, size, self.id);
            }

            InstanceMessage::Navigate(url) => {
                //TODO
            }

            InstanceMessage::Back => {
                //TODO
            }

            InstanceMessage::Forward => {
                //TODO
            }

            InstanceMessage::Reload => {
                let el = self.el.clone();

                task::spawn_local(self.data.reload(el));
            }

            InstanceMessage::Close => {
                self.rx.close();
                self.irx.close();
            }

            InstanceMessage::Debug(event) => {
                //TODO
            }
        }
    }
}

pub struct InstanceHandle {
    pub tx: Sender<InstanceMessage>,
}

pub enum InstanceMessage {
    Redraw(SizeU32),

    Navigate(Url),
    Back,
    Forward,
    Reload,
    Close,

    // Input(InputEvent), //TODO
    Debug(DebugEvent),
}

#[derive(Clone)]
struct El<C: ModuleConfiguration>(Sender<InternalInstanceMessage<C>>);

impl<C: ModuleConfiguration> EventLoopHandle<C> for El<C> {
    fn redraw(&self) {
        let send = self.0.clone();

        spawn_blocking(move || {
            let _ = send.blocking_send(InternalInstanceMessage::Redraw);
        });
    }

    fn add_img_cache(&self, url: String, buf: ImageBuffer<C::RenderBackend>, size: Option<SizeU32>) {
        let send = self.0.clone();

        spawn_blocking(move || {
            let _ = send.blocking_send(InternalInstanceMessage::Image(Url::parse(&url).unwrap(), buf, size));
        });
    }

    fn reload_from(&self, rt: C::RenderTree) {
        let send = self.0.clone();

        spawn_blocking(move || {
            let _ = send.blocking_send(InternalInstanceMessage::ReloadFrom(rt));
        });
    }
}

pub enum InternalInstanceMessage<C: HasTreeDrawer> {
    Image(Url, ImageBuffer<C::RenderBackend>, Option<SizeU32>),
    Redraw,
    ReloadFrom(C::RenderTree),
}

pub enum DebugEvent {
    SendNodes(SyncSender<NodeDesc>),
    SelectElement(usize),
    Info(usize, SyncSender<NodeDesc>),
    Deselect,
    Toggle,
    Enable,
    Disable,
    ClearBuffers,
}
