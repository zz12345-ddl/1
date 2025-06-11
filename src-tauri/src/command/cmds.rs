use crate::command::model::find_port;
use crate::command::model::ServerState;
use base64::prelude::*;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{path::BaseDirectory, utils::config::WindowConfig, AppHandle, LogicalSize, Manager};
use tauri_plugin_http::reqwest;
use warp::Filter;

#[tauri::command]
pub async fn start_server(
    state: tauri::State<'_, Arc<Mutex<ServerState>>>,
    path: String,
) -> Result<u16, String> {
    let mut state = state.lock().unwrap();
    if state.server_handle.is_some() {
        return Err("Server is already running".into());
    }
    let path_clone = path.clone();
    let port = find_port().unwrap();
    println!("port: {}", port);
    let server_handle = tokio::spawn(async move {
        let route = warp::fs::dir(path_clone)
            .map(|reply| {
                warp::reply::with_header(
                    reply,
                    "Cache-Control",
                    "no-store, no-cache, must-revalidate, max-age=0",
                )
            })
            .map(|reply| warp::reply::with_header(reply, "Vary", "*"))
            .map(|reply| warp::reply::with_header(reply, "Surrogate-Control", "no-store"))
            .map(|reply| warp::reply::with_header(reply, "Pragma", "no-cache"))
            .map(|reply| warp::reply::with_header(reply, "Expires", "0"));
        warp::serve(route).run(([127, 0, 0, 1], port)).await;
    });
    state.server_handle = Some(server_handle);
    Ok(port)
}

#[tauri::command]
pub async fn stop_server(state: tauri::State<'_, Arc<Mutex<ServerState>>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if let Some(handle) = state.server_handle.take() {
        handle.abort();
        Ok(())
    } else {
        Err("Server is not running".into())
    }
}

#[tauri::command]
pub async fn open_window(
    handle: AppHandle,
    app_url: String,
    app_name: String,
    platform: String,
    user_agent: String,
    resize: bool,
    width: f64,
    height: f64,
    js_content: String,
) {
    let window_label = "previewWeb";
    if let Some(existing_window) = handle.get_webview_window(window_label) {
        if resize {
            let new_size = LogicalSize::new(width, height);
            match existing_window.set_size(new_size) {
                Ok(_) => println!("Window resized to {}x{}", width, height),
                Err(e) => eprintln!("Failed to resize window: {}", e),
            }
        } else {
            existing_window.close().unwrap();
            println!("Existing window closed.");
            let start = Instant::now();
            while handle.get_webview_window(window_label).is_some() {
                if start.elapsed().as_secs() > 2 {
                    println!("Window close took too long. Aborting.");
                    return;
                }
                std::thread::yield_now();
            }
        }
    }
    println!("Opening docs in external window: {}, {}", app_url, platform);
    let resource_path = handle
        .path()
        .resolve("data/custom.js", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut custom_js = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    custom_js.read_to_string(&mut contents).unwrap();
    contents += js_content.as_str();
    println!("js file contents: {}", contents);
    if !resize {
        let _window = tauri::WebviewWindowBuilder::new(
            &handle,
            "previewWeb", /* the unique window label */
            tauri::WebviewUrl::External(app_url.parse().unwrap()),
        )
        .title(app_name)
        .inner_size(width, height)
        .user_agent(user_agent.as_str())
        .center()
        .build()
        .unwrap();
    }
}

#[tauri::command]
pub async fn preview_from_config(
    handle: AppHandle,
    resize: bool,
    config: WindowConfig,
    js_content: String,
    devbug: bool,
) {
    let window_label = "PreView";
    if let Some(existing_window) = handle.get_webview_window(window_label) {
        if resize {
            let new_size = LogicalSize::new(config.width, config.height);
            match existing_window.set_size(new_size) {
                Ok(_) => println!("Window resized to {}x{}", config.width, config.height),
                Err(e) => eprintln!("Failed to resize window: {}", e),
            }
        } else {
            existing_window.close().unwrap();
            let start = Instant::now();
            while handle.get_webview_window(window_label).is_some() {
                if start.elapsed().as_secs() > 2 {
                    println!("Window close took too long. Aborting.");
                    return;
                }
                std::thread::yield_now();
            }
        }
    }
    let mut contents = String::new();
    if devbug {
        contents += include_str!("../../data/vconsole.min.js");
        contents += "var vConsole = new window.VConsole();";
    }
    // custom js
    contents += js_content.as_str();
    if !resize {
        let _window = tauri::WebviewWindowBuilder::from_config(&handle, &config)
            .unwrap()
            .initialization_script(contents.as_str())
            .build()
            .unwrap();
    }
}

#[tauri::command]
pub async fn update_build_file(handle: tauri::AppHandle, name: String, body: String) -> String {
    let resource_path = handle
        .path()
        .resolve("data/build.yml", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut build_file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    build_file.read_to_string(&mut contents).unwrap();
    contents = contents
        .replace("PROJECTNAME", name.as_str())
        .replace("RELEASEBODY", body.as_str());
    // println!("Updated build file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn update_config_file(
    handle: tauri::AppHandle,
    name: String,
    version: String,
    id: String,
    ascii: bool,
    window_config: String,
    tauri_api: bool,
) -> String {
    let resource_path = handle
        .path()
        .resolve("data/config.json", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut config_file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    contents = contents
        .replace("PROJECTNAME", name.as_str())
        .replace("PROJECTVERSION", version.as_str())
        .replace("PROJECTID", id.as_str());
    if tauri_api {
        contents = contents.replace("-2", r#"true"#);
    } else {
        contents = contents.replace("-2", r#"false"#);
    }
    if ascii {
        contents = contents.replace("-3", r#""all""#);
    } else {
        contents = contents.replace("-3", r#"["deb", "appimage", "nsis", "app", "dmg"]"#);
    }
    contents = contents.replace("-1", window_config.as_str());
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn update_config_json(
    handle: tauri::AppHandle,
    name: String,
    version: String,
    id: String,
    ascii: bool,
) -> String {
    let resource_path = handle
        .path()
        .resolve("data/config.json", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut config_file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    contents = contents
        .replace("PROJECTNAME", name.as_str())
        .replace("PROJECTVERSION", version.as_str())
        .replace("PROJECTID", id.as_str());
    if ascii {
        contents = contents.replace("-3", r#""all""#);
    } else {
        contents = contents.replace("-3", r#"["deb", "appimage", "nsis", "app", "dmg"]"#);
    }
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn update_cargo_file(
    handle: tauri::AppHandle,
    name: String,
    version: String,
    desc: String,
    debug: bool,
) -> String {
    let resource_path = handle
        .path()
        .resolve("data/cargo.txt", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut config_file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    contents = contents
        .replace("PROJECTNAME", name.as_str())
        .replace("PROJECTVERSION", version.as_str())
        .replace("PROJECTDESC", desc.as_str());
    if debug {
        // "shell-open", "devtools"
        contents = contents.replace("-3", r#""protocol-asset", "devtools""#);
    } else {
        contents = contents.replace("-3", r#""protocol-asset""#);
    }
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn update_main_rust(
    handle: tauri::AppHandle,
    app_url: String,
    app_name: String,
    user_agent: String,
    width: f64,
    height: f64,
) -> String {
    let resource_path = handle
        .path()
        .resolve("data/lib.rs", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut main_rust = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    main_rust.read_to_string(&mut contents).unwrap();
    contents = contents
        .replace("PROJECTNAME", app_name.as_str())
        .replace("PROJECTURL", app_url.as_str())
        .replace("PROJECTUSERAGENT", user_agent.as_str())
        .replace("-1", width.to_string().as_str())
        .replace("-2", height.to_string().as_str());
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn rust_main_window(handle: tauri::AppHandle, config: String) -> String {
    let resource_path = handle
        .path()
        .resolve("data/main.rs", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut main_rust = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    main_rust.read_to_string(&mut contents).unwrap();
    // test replace
    contents = contents.replace("WINDOWCONFIG", config.as_str());
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn rust_lib_window(handle: tauri::AppHandle, config: String) -> String {
    let resource_path = handle
        .path()
        .resolve("data/lib.rs", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut main_rust = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    main_rust.read_to_string(&mut contents).unwrap();
    contents = contents.replace("WINDOWCONFIG", config.as_str());
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

#[tauri::command]
pub async fn get_custom_js(handle: tauri::AppHandle) -> String {
    let resource_path = handle
        .path()
        .resolve("data/custom.js", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut custom_js = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    custom_js.read_to_string(&mut contents).unwrap();
    return contents;
}

#[tauri::command]
pub async fn update_custom_js(_: tauri::AppHandle, js_content: String) -> String {
    // let resource_path = handle
    //     .path()
    //     .resolve("data/custom.js", BaseDirectory::Resource)
    //     .expect("failed to resolve resource");
    // let mut custom_js = std::fs::File::open(&resource_path).unwrap();
    // let mut contents = String::new();
    // custom_js.read_to_string(&mut contents).unwrap();
    // contents += js_content.as_str();
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(js_content);
    return encoded_contents;
}

#[tauri::command]
pub async fn content_to_base64(_: tauri::AppHandle, content: String) -> String {
    // println!("Updated config file: {}", contents);
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(content);
    return encoded_contents;
}

#[tauri::command]
pub async fn open_url(_: tauri::AppHandle, url: String) {
    open::that(url).unwrap();
}

// open devtools
#[tauri::command]
pub async fn open_devtools(handle: AppHandle) {
    if let Some(_) = handle.get_webview_window("main") {
        println!("open devtools");
        // existing_window.open_devtools();
    }
}

#[tauri::command]
pub async fn update_init_rs(
    handle: tauri::AppHandle,
    config: String,
    state: bool,
    injectjq: bool,
) -> String {
    let resource_path = handle
        .path()
        .resolve("data/init.rs", BaseDirectory::Resource)
        .expect("failed to resolve resource");
    let mut main_rust = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    main_rust.read_to_string(&mut contents).unwrap();
    contents = contents.replace("WINDOWCONFIG", config.as_str());
    // 替换state
    if state {
        println!("state: true");
    } else {
        contents = contents.replace("if true {", "if false {");
    }
    // 替换injectjq
    if injectjq {
        contents = contents.replace(
            r#".initialization_script(include_str!("../../data/custom.js"))"#,
            r#".initialization_script(include_str!("../../data/jquery.min.js")).initialization_script(include_str!("../../data/custom.js"))"#,
        );
    }
    // The new file content, using Base64 encoding
    let encoded_contents = BASE64_STANDARD.encode(contents);
    return encoded_contents;
}

// #[tauri::command]
// pub async fn download_file_by_binary(
//     app: AppHandle,
//     params: BinaryDownloadParams,
// ) -> Result<(), String> {
//     let window: Window = app.get_window("pake").unwrap();
//     show_toast(&window, &get_download_message(MessageType::Start));
//     let output_path = api::path::download_dir().unwrap().join(params.filename);
//     let file_path = check_file_or_append(output_path.to_str().unwrap());
//     let download_file_result = fs::write(file_path, &params.binary);
//     match download_file_result {
//         Ok(_) => {
//             show_toast(&window, &get_download_message(MessageType::Success));
//             Ok(())
//         }
//         Err(e) => {
//             show_toast(&window, &get_download_message(MessageType::Failure));
//             Err(e.to_string())
//         }
//     }
// }

// following user
#[tauri::command]
pub async fn support_pp(_: AppHandle, token: String) {
    let client = reqwest::Client::builder().build().unwrap();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    // user agent
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap(),
    );
    let request = client
        .request(
            reqwest::Method::PUT,
            &format!("https://api.github.com/user/following/{}", "your-username"),
        )
        .headers(headers);
    let response = request.send().await.unwrap();
    let _ = response.text().await.unwrap();
    // println!("res_follow = {res_follow:?}");

    // star repo
    let client = reqwest::Client::builder().build().unwrap();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    // user agent
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap(),
    );
    let request = client
        .request(
            reqwest::Method::PUT,
            &format!("https://api.github.com/user/starred/{}/{}", "your-username", "WebAppBuilder"),
        )
        .headers(headers.clone());
    let response = request.send().await.unwrap();
    let _ = response.text().await.unwrap();
    // star pakeplus-ios
    // let request = client
    //     .request(
    //         reqwest::Method::PUT,
    //         "https://api.github.com/user/starred/Sjj1024/PakePlus-iOS",
    //     )
    //     .headers(headers.clone());
    // let response = request.send().await.unwrap();
    // let _ = response.text().await.unwrap();
    // star pakeplus-android
    let request = client
        .request(
            reqwest::Method::PUT,
            &format!("https://api.github.com/user/starred/{}/{}-Android", "your-username", "WebAppBuilder"),
        )
        .headers(headers.clone());
    let response = request.send().await.unwrap();
    let _ = response.text().await.unwrap();
}
