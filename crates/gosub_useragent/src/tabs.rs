use gosub_render_backend::RenderBackend;
use gosub_renderer::draw::SceneDrawer;
use slotmap::{DefaultKey, SlotMap};

pub struct Tabs<D: SceneDrawer<B>, B: RenderBackend> {
    pub tabs: SlotMap<DefaultKey, Tab<D, B>>,
    pub active: TabID,
}

impl<D: SceneDrawer<B>, B: RenderBackend> Tabs<D, B> {
    pub fn new(initial: Tab<D, B>) -> Self {
        let mut tabs = SlotMap::new();
        let active = TabID(tabs.insert(initial));

        Self { tabs, active }
    }

    pub fn add_tab(&mut self, tab: Tab<D, B>) -> TabID {
        TabID(self.tabs.insert(tab))
    }

    pub fn remove_tab(&mut self, id: TabID) {
        self.tabs.remove(id.0);
    }

    pub fn activate_tab(&mut self, id: TabID) {
        self.active = id;
    }
}

pub struct Tab<D: SceneDrawer<B>, B: RenderBackend> {
    pub title: String,
    pub url: String,
    pub data: D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabID(pub(crate) DefaultKey);
