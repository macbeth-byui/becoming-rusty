
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Vec2;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::fs::File;
use std::io;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Gui(eframe::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Json(e) => write!(f,"JSON Error: {}", e),
            AppError::Gui(e) => write!(f,"GUI Error: {}", e)
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

impl From<eframe::Error> for AppError {
    fn from(e: eframe::Error) -> Self {
        AppError::Gui(e)
    }
}


#[derive(Debug, Clone, PartialEq)]
enum Major {
    Cs,
    Swe,
}

impl fmt::Display for Major {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Major::Cs => write!(f, "Computer Science"),
            Major::Swe => write!(f, "Software Engineering")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Catalog {
    C2022,
    C2023,
    C2024,
}

impl fmt::Display for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Catalog::C2022 => write!(f, "2022"),
            Catalog::C2023 => write!(f, "2023"),
            Catalog::C2024 => write!(f, "2024")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Course {
    id : String,
    title : String,
    description : String,
    credits : String,
    #[serde(default)]
    prerequisites : String,
}

#[derive(Debug, Clone)]
struct Cs2024Plan {
    religion_1 : Option<String>,
    religion_2 : Option<String>,
    religion_3 : Option<String>,
    quantitative : Option<String>,
    writing : Option<String>,
}

impl Cs2024Plan {
    fn new() -> Cs2024Plan {
        Cs2024Plan { 
            religion_1 : None,
            religion_2 : None,
            religion_3 : None,
            quantitative : None,
            writing : None,
        }
    }
}

enum CoursePlan {
    Cs2024(Cs2024Plan),
    Cs2023(),
}

struct OakTreeApp {
    major: Major,
    catalog: Catalog,
    courses: BTreeMap<String, Course>,
    selected_course: Option<String>,
    course_plan : CoursePlan,
}

// Load Course JSON => List of Courses
// Process DA PDF => List of Taken Courses (parse substitute & waived)
// Display CS Degree current Catalog Year with drop down boxes if > 1
// Show courses taken
// Show metrics
// Show declared degree and catalog
// Display class when you select one
// Display class when you click the spyglass button 
// Drop down for all courses
// Hyperlink for pre-reqs
// Allow you to show what it looks like if you change between CS & SWE
// When you have to select a certificate, have a checkbox that will move other courses to other areas
// Print out report
// Save advising notes 

impl OakTreeApp {
    fn new() -> Result<Box<OakTreeApp>, AppError> {
        let file = File::open("courses.json")?;
        let mut reader = io::BufReader::new(file);
        let course_list : Vec<Course> = serde_json::from_reader(&mut reader)?;
        let courses : BTreeMap<String, Course> = course_list.into_iter().map(|c| (c.id.clone(), c)).collect();
        Ok(Box::new(OakTreeApp { 
            major : Major::Cs, 
            catalog : Catalog::C2024, 
            courses,
            selected_course : None,
            course_plan : CoursePlan::Cs2024(Cs2024Plan::new()),
         }))
    }

    fn course_line(id : &str, courses : &BTreeMap<String,Course>, indent : u8) -> String {
        OakTreeApp::line_indent(format!("{} - {} ({})", id, courses[id].title, courses[id].credits).as_str(), indent)
    }

    fn line_indent(text : &str, indent : u8) -> String {
        format!("{}{}"," ".repeat(indent as usize * 6), text)
    }

    fn cs_2024(&mut self, ui : &mut egui::Ui) {
        if let CoursePlan::Cs2024(plan) = &mut self.course_plan {
            ui.heading("Generals");
            ui.add(egui::Separator::default());
            ui.add_space(12.0);
            ui.heading("First Year");
            ui.label(OakTreeApp::course_line("BYUI101", &self.courses, 1));
            ui.add_space(12.0);
            ui.heading("Religion");
            ui.label(OakTreeApp::course_line("REL200C", &self.courses, 1));
            ui.label(OakTreeApp::course_line("REL225C", &self.courses, 1));
            ui.label(OakTreeApp::course_line("REL250C", &self.courses, 1));
            ui.label(OakTreeApp::course_line("REL275C", &self.courses, 1));
            ui.label(OakTreeApp::line_indent("Religion Electives (6)", 1));
            ui.horizontal(|ui| {
                ui.label(OakTreeApp::line_indent("Choice 1", 2));
                egui::ComboBox::from_id_source("religion_1")
                    .selected_text(plan.religion_1.clone().unwrap_or("Select...".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut plan.religion_1, None, "Select...".to_string());
                        for course in self.courses.keys() {
                            if course.starts_with("REL") {
                                ui.selectable_value(&mut self.selected_course, 
                                    Some(course.clone()), 
                                    OakTreeApp::course_line(course, &self.courses, 0));
                            }
                        }
                });
            });
            ui.horizontal(|ui| {
                ui.label(OakTreeApp::line_indent("Choice 2", 2));
                egui::ComboBox::from_id_source("religion_2")
                    .selected_text(plan.religion_2.clone().unwrap_or("Select...".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut plan.religion_2, None, "Select...".to_string());
                        for course in self.courses.keys() {
                            if course.starts_with("REL") {
                                ui.selectable_value(&mut self.selected_course, 
                                    Some(course.clone()), 
                                    OakTreeApp::course_line(course, &self.courses, 0));
                            }
                        }
                });
            });            
            ui.horizontal(|ui| {
                ui.label(OakTreeApp::line_indent("Choice 3", 2));
                egui::ComboBox::from_id_source("religion_3")
                    .selected_text(plan.religion_3.clone().unwrap_or("Select...".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut plan.religion_3, None, "Select...".to_string());
                        for course in self.courses.keys() {
                            if course.starts_with("REL") {
                                ui.selectable_value(&mut self.selected_course, 
                                    Some(course.clone()), 
                                    OakTreeApp::course_line(course, &self.courses, 0));
                            }
                        }
                });
            });                    
            ui.add_space(12.0);
            ui.heading("Quantitative Reasoning");
            ui.horizontal(|ui| {
                ui.label(OakTreeApp::line_indent("Pick 1", 1));
                egui::ComboBox::from_id_source("quantitative")
                    .selected_text(plan.quantitative.clone().unwrap_or("Select...".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut plan.quantitative, None, "Select...".to_string());
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("ECON215".to_string()), 
                            OakTreeApp::course_line("ECON215", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH108X".to_string()), 
                            OakTreeApp::course_line("MATH108X", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH112X".to_string()),
                            OakTreeApp::course_line("MATH112X", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH119".to_string()), 
                            OakTreeApp::course_line("MATH119", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH221A".to_string()), 
                            OakTreeApp::course_line("MATH221A", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH221B".to_string()), 
                            OakTreeApp::course_line("MATH221B", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH221C".to_string()), 
                            OakTreeApp::course_line("MATH221C", &self.courses, 0));
                        ui.selectable_value(&mut plan.quantitative, 
                            Some("MATH221D".to_string()), 
                            OakTreeApp::course_line("MATH221D", &self.courses, 0));
                });
            }); 

            ui.add_space(12.0);
            ui.heading("Writing");    
            ui.label(OakTreeApp::course_line("ENG150", &self.courses, 1));               

            ui.horizontal(|ui| {
                ui.label(OakTreeApp::line_indent("Pick 1", 1));
                egui::ComboBox::from_id_source("writing")
                    .selected_text(plan.writing.clone().unwrap_or("Select...".to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut plan.writing, None, "Select...".to_string());
                        ui.selectable_value(&mut plan.writing, 
                            Some("BUS301".to_string()), 
                            OakTreeApp::course_line("BUS301", &self.courses, 0));
                        ui.selectable_value(&mut plan.writing, 
                            Some("ENG301".to_string()), 
                            OakTreeApp::course_line("ENG301", &self.courses, 0));
                    });
                });
        }

    }

}

impl eframe::App for OakTreeApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .min_width(300.0)
            .show(ctx, |ui| {

                ui.vertical_centered(|ui| {
                    ui.heading("Oak Tree");
                    ui.add(egui::Image::new(egui::include_image!("oaktree.png"))
                        .max_height(200.0)
                        .max_width(200.0)
                    );
                });
                    
                ui.add(egui::Separator::default());
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Major: ");
                    egui::ComboBox::from_id_source("major")
                        .selected_text(format!("{}", self.major))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.major, Major::Cs, Major::Cs.to_string());
                            ui.selectable_value(&mut self.major, Major::Swe, Major::Swe.to_string());
                    });
                });
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Catalog: ");
                    egui::ComboBox::from_id_source("catalog")
                        .selected_text(format!("{}", self.catalog))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.catalog, Catalog::C2022, Catalog::C2022.to_string());
                            ui.selectable_value(&mut self.catalog, Catalog::C2023, Catalog::C2023.to_string());
                            ui.selectable_value(&mut self.catalog, Catalog::C2024, Catalog::C2024.to_string());
                    });
                });
                ui.add_space(12.0);
                ui.add(egui::Separator::default());
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Course: ");
                    egui::ComboBox::from_id_source("courses")
                        .selected_text(self.selected_course.clone().unwrap_or("Select...".to_string()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_course, None, "Select...".to_string());
                            for course in self.courses.keys() {
                                ui.selectable_value(&mut self.selected_course, 
                                    Some(course.clone()), 
                                    course.clone());
                            }
                        });
                });
                ui.add_space(12.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(selected_course) = &self.selected_course {
                        let course = &self.courses[selected_course];
                        ui.vertical_centered(|ui| {
                            ui.heading(format!("{} - {}",course.id, course.title));
                        });
                        ui.add(egui::Separator::default());
                        ui.add_space(12.0);
                        ui.heading("Description");
                        ui.label(&course.description);
                        ui.add_space(12.0);
                        ui.heading("Credits");
                        ui.label(&course.credits);
                        ui.add_space(12.0);
                        ui.heading("Pre-Requisites");
                        ui.label(&course.prerequisites);
                    }
                });
                
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.cs_2024(ui);            
        });
    }
}

fn main() -> Result<(), AppError> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    let app = OakTreeApp::new()?;
    eframe::run_native(
        "Oak Tree",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(app)
        }),
    )?;
    println!("Goodbye!");
    Ok(())
}

