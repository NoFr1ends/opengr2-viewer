use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use opengr2::GrannyFile;
use tracing::info;
use crate::element_ui::element_ui;

pub struct TemplateApp {
    label: String,
    value: f32,

    file: Option<GrannyFile>,
    file_name: String,

    tx: Sender<Option<FileOpened>>,
    rx: Receiver<Option<FileOpened>>
}

pub struct FileOpened {
    data: Vec<u8>,
    file_name: String
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            file: None,
            file_name: "".to_owned(),
            tx,
            rx
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn open_file(tx: Sender<Option<FileOpened>>) {
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(path) = rfd::AsyncFileDialog::new().add_filter("Granny2 File", &["gr2"]).pick_file().await {
                let data = path.read().await;

                info!("Read {} bytes", data.len());
                tx.send(Some(FileOpened {
                    data,
                    file_name: path.file_name()
                })).unwrap();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_file(tx: Sender<Option<FileOpened>>) {
        if let Some(path) = rfd::FileDialog::new().add_filter("Granny2 File", &["gr2"]).pick_file() {
            let mut file = File::open(path).unwrap();
            let mut data = Vec::new();
            let size = file.read_to_end(&mut data).unwrap();

            info!("Read {} bytes", size);

            tx.send(Some(FileOpened {
                data,
                file_name: path.file_name().unwrap().to_str().unwrap().to_owned()
            })).unwrap();
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            file,
            file_name,
            tx,
            rx
        } = self;

        if let Ok(evt) = rx.try_recv() {
            if let Some(evt) = evt {
                info!("Received granny file data!");
                if let Some(granny_file) = GrannyFile::load_from_bytes(&evt.data) {
                    *file = Some(granny_file);
                    *file_name = evt.file_name;
                } else {
                    // todo show error!
                }
            } else {
                // todo show error!
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        TemplateApp::open_file(tx.clone());
                    }

                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if let Some(file) = file {
                ui.heading(format!("{}", file_name));

                egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    for element in &file.root_elements {
                        element_ui(ui, element);
                    }
                });
            } else {
                ui.label("No file open");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}