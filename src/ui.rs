use eframe;
use serde_json::json;

use ehttp;
use std::sync::Arc;
use std::sync::RwLock;

pub struct MyApp {
    received_messages: Arc<RwLock<Vec<String>>>, //channel could have better performance.
    user_input: String,
}

impl<'a> MyApp {
    pub fn new() -> Self {
        Self {
            received_messages: Arc::new(RwLock::new(Vec::<String>::new())),
            user_input: String::default(),
        }
    }
}

impl<'a> eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hello!");
            let received_messages = self.received_messages.read().unwrap();
            for msg in received_messages.iter() {
                ui.label(msg);
            }
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.user_input);
                if ui.button("Send").clicked() {
                    fetch(
                        self.user_input.clone(),
                        self.received_messages.clone(),
                        ctx.clone(),
                    );
                    self.user_input.clear();
                }
            });
        });
    }
}

fn fetch(user_input: String, received_messages: Arc<RwLock<Vec<String>>>, ctx: egui::Context) {
    let message = json!({
                     "query": user_input,
                     "topic": "jedi",
    });

    let request = ehttp::Request::post(
        "http://localhost:8080/query",
        serde_json::to_vec(&message).unwrap(),
    );
    ehttp::fetch(request, move |response| {
        match response {
            Ok(resp) => {
                //TODO: add messages;
                resp.text().map(|txt| {
                    log::info!("{}", txt);
                    received_messages.write().unwrap().push(txt.to_string());
                });
                ctx.request_repaint();
            }
            Err(e) => {
                log::info!("{}",e);
                //println!("{:?}", String::from(e))
            }
        }
    });
}
