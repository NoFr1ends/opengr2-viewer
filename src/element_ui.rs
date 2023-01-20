use opengr2::parser::{Element, ElementType};

fn element_content(ui: &mut egui::Ui, element: &ElementType) {
    match element {
        ElementType::Reference(elements) => {
            for e in elements {
                element_ui(ui, e);
            }
        }
        ElementType::ArrayOfReferences(array) => {
            for (i, elements) in array.iter().enumerate() {
                egui::CollapsingHeader::new(format!("[{}]", i)).show(ui, |ui| {
                    for e in elements {
                        element_ui(ui, e);
                    }
                });
            }
        }
        ElementType::VariantReference => {
            ui.label("Currently not supported!");
        }
        ElementType::String(val) => {
            ui.label(val);
        }
        ElementType::F32(float) => {
            ui.label(format!("{}", float));
        }
        ElementType::I32(integer) => {
            ui.label(format!("{}", integer));
        }
        ElementType::Transform(_) => {}
        ElementType::Array(elements) => {
            for e in elements {
                element_content(ui, e);
            }
        }
    }
}

pub fn element_ui(ui: &mut egui::Ui, element: &Element) {
    egui::CollapsingHeader::new(element.name.clone()).show(ui, |ui| {
        element_content(ui, &element.element);
    });
}