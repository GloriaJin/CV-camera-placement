use druid::widget::{Button, Column, Label, TextBox};
use druid::{
    AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, commands, Command,
};
use std::process::{Command as SysCommand, Stdio};
use std::io::Read;

#[derive(Debug, Clone, Data, Lens)]
struct AppState {
    file_path: String,
    fov: f64,
    resolution: String,
    processing_result: String,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("STL File Viewer"))
        .window_size((300.0, 250.0));

    let initial_state = AppState {
        file_path: String::new(),
        fov: 0.0,
        resolution: String::new(),
        processing_result: String::new(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<AppState> {
    let file_path_label = Label::new("Enter the path to the STL file:");
    let file_path_entry = TextBox::new()
        .lens(AppState::file_path)
        .expand_width();

    let fov_label = Label::new("Field of View (FOV):");
    let fov_entry = TextBox::new()
        .lens(AppState::fov)
        .expand_width();

    let resolution_label = Label::new("Resolution:");
    let resolution_entry = TextBox::new()
        .lens(AppState::resolution)
        .expand_width();

    let load_button = Button::new("Load STL")
        .on_click(|ctx, data: &mut AppState, _env| {
            let stl_path = &data.file_path;
            let fov = &data.fov.to_string();
            let resolution = &data.resolution;

            // Run the Python script
            if let Ok(output) = launch_python_script(stl_path, fov, resolution) {
                data.processing_result = output;
            } else {
                data.processing_result = String::from("Error launching Python script");
            }

            ctx.request_update();
        });

    let result_label = Label::dynamic(|data: &AppState, _env| data.processing_result.clone());

    Column::new()
        .spacing(10.0)
        .padding(10.0)
        .with_child(file_path_label)
        .with_child(file_path_entry)
        .with_child(fov_label)
        .with_child(fov_entry)
        .with_child(resolution_label)
        .with_child(resolution_entry)
        .with_child(load_button)
        .with_child(result_label)
}

fn launch_python_script(stl_path: &str, fov: &str, resolution: &str) -> Result<String, std::io::Error> {
    let output = Arc::new(Mutex::new(String::new()));

        let output_clone = output.clone();
        let stl_path = stl_path.to_string();
        let fov = fov.to_string();
        let resolution = resolution.to_string();

        thread::spawn(move || {
            let mut command = SysCommand::new("python")
                .arg("stl_viewer.py")
                .arg(&stl_path)
                .arg(&fov)
                .arg(&resolution)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start Python script");

            let stdout = command.stdout.take().expect("Failed to open stdout");
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let mut output = output_clone.lock().unwrap();
                let line = line.expect("Failed to read line");
                output.push_str(&line);
                output.push('\n');
            }
        });

        // return output from python script
        let output = output.lock().unwrap().clone();
        Ok(output)
}
