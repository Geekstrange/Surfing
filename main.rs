use std::{
    env, fs,
    io::{self, Write, Read},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use ctrlc;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;

// ANSI颜色代码
const RED_BG: &str = "\x1b[41;37m";
const YELLOW_BG: &str = "\x1b[43;34m";
const GREEN_BG: &str = "\x1b[42;30m";
const CYAN_BG: &str = "\x1b[46;37m";
const RED_WD: &str = "\x1b[31m";
const YELLOW_WD: &str = "\x1b[33m";
const GREEN_WD: &str = "\x1b[32m";
const CYAN_WD: &str = "\x1b[36m";
const BLINK: &str = "\x1b[5m";
const ITALIC: &str = "\x1b[3m";
const LB: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

const MAX_RETRY: u32 = 3;

struct DownloadInfo {
    current_size: u64,
    speed: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("使用方法: {} <下载URL> <文件名> [保存路径]", args[0]);
        std::process::exit(1);
    }

    let download_url = &args[1];
    let filename = &args[2];
    let download_dir = if args.len() > 3 { &args[3] } else { "." };

    // 检查文件大小
    let file_size = get_file_size(download_url);

    if file_size.is_some() {
        if let Err(e) = real_progress_bar(download_url, filename, download_dir) {
            eprintln!("下载失败: {}", e);
            std::process::exit(1);
        }
    } else {
        if let Err(e) = surfing_progress_bar(download_url, filename, download_dir) {
            eprintln!("下载失败: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_file_size(url: &str) -> Option<u64> {
    let client = Client::new();
    let response = match client.head(url).send() {
        Ok(r) => r,
        Err(_) => return None,
    };

    response.headers().get(CONTENT_LENGTH)
        .and_then(|length| length.to_str().ok())
        .and_then(|size_str| size_str.parse().ok())
}

fn hide_cursor() {
    print!("\x1b[?25l");
    io::stdout().flush().unwrap();
}

fn show_cursor() {
    print!("\x1b[?25h");
    io::stdout().flush().unwrap();
}

fn surfing_progress_bar(url: &str, filename: &str, download_dir: &str) -> Result<(), String> {
    let download_info = Arc::new(Mutex::new(DownloadInfo { current_size: 0, speed: 0 }));
    let running = Arc::new(AtomicBool::new(true));
    let file_path = Path::new(download_dir).join(filename);
    let file_path_str = file_path.to_str().ok_or("无效文件路径")?.to_string();

    // Ctrl+C处理
    let running_clone = running.clone();
    let file_path_clone = file_path_str.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        let current_size = fs::metadata(&file_path_clone)
            .map(|m| m.len())
            .unwrap_or(0);
        print!("\r\x1b[K {}{}[!]{} 用户中断 {}{}{}(已下载:{}kb){}\n",
               BLINK, CYAN_WD, RESET, ITALIC, LB, CYAN_WD, current_size / 1024, RESET);
        show_cursor();
        let _ = fs::remove_file(&file_path_clone);
        std::process::exit(1);
    }).map_err(|e| format!("设置Ctrl-C处理程序失败: {}", e))?;

    for attempt in 1..=MAX_RETRY {
        if !running.load(Ordering::SeqCst) { break; }

        let download_info_clone = download_info.clone();
        let running_clone = running.clone();
        let file_path_clone = file_path_str.clone();

        // 文件大小监控线程
        let monitor_thread = thread::spawn(move || {
            let mut prev_size = 0;
            let start_time = Instant::now();

            while running_clone.load(Ordering::SeqCst) {
                if let Ok(metadata) = fs::metadata(&file_path_clone) {
                    let current_size = metadata.len();
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        current_size.saturating_sub(prev_size) as f64 / elapsed
                    } else {
                        0.0
                    };

                    {
                        let mut info = download_info_clone.lock().unwrap();
                        info.current_size = current_size;
                        info.speed = speed as u64;
                    }
                    prev_size = current_size;
                }
                thread::sleep(Duration::from_millis(500));
            }
        });

        // 波浪动画线程
        let download_info_clone = download_info.clone();
        let running_clone = running.clone();
        let animation_thread = thread::spawn(move || {
            wave_animation(attempt, download_info_clone, running_clone);
        });

        // 下载文件
        let download_result = download_file(url, &file_path_str);

        running.store(false, Ordering::SeqCst);
        let _ = monitor_thread.join();
        let _ = animation_thread.join();

        match download_result {
            Ok(_) => {
                let current_size = fs::metadata(&file_path_str)
                    .map(|m| m.len())
                    .unwrap_or(0);
                print!("\r\x1b[K 下载完成 共计:{}kb\n", current_size / 1024);
                show_cursor();
                return Ok(());
            }
            Err(e) => {
                print!("\r\x1b[K 下载失败: {}", e);
                let _ = fs::remove_file(&file_path_str);
                show_cursor();

                if attempt < MAX_RETRY {
                    retry_animation();
                    running.store(true, Ordering::SeqCst);
                }
            }
        }
    }

    print!("\r\x1b[K已达最大重试次数\n");
    let _ = fs::remove_file(&file_path_str);
    show_cursor();
    Err("下载失败".into())
}

fn wave_animation(
    attempt: u32,
    download_info: Arc<Mutex<DownloadInfo>>,
    running: Arc<AtomicBool>,
) {
    hide_cursor();
    let wave_blocks = "▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▂▃▄▅▆▇█▇▆▅▄▃▂▁";
    let wave_chars: Vec<char> = wave_blocks.chars().collect();
    let mut positions = vec![0i32, -2, 2];
    let mut directions = vec![1i32, -1, 1];

    while running.load(Ordering::SeqCst) {
        let mut core_line = String::new();

        for i in 0..wave_chars.len() {
            let mut max_height = 0;
            for &pos in &positions {
                let distance = ((i as i32 - pos + wave_chars.len() as i32) % wave_chars.len() as i32).abs();
                let distance = if distance > wave_chars.len() as i32 / 2 {
                    wave_chars.len() as i32 - distance
                } else {
                    distance
                };
                let height = wave_chars.len() as i32 - distance;
                if height > max_height {
                    max_height = height;
                }
            }

            let mut index = (max_height * wave_chars.len() as i32 / (wave_chars.len() as i32 + 2)) as usize;
            if index >= wave_chars.len() {
                index = wave_chars.len() - 1;
            }
            core_line.push(wave_chars[index]);
        }

        let info_text = {
            let info = download_info.lock().unwrap();
            format!("已下载:{}kb 速度:{}kbps", info.current_size / 1024, info.speed / 1024)
        };

        let full_line = format!(" Surfing:{}[{}]{} {} 尝试下载(第 {} 次)",
                              CYAN_BG, core_line, RESET, info_text, attempt);
        print!("\r\x1b[K{}", full_line);
        io::stdout().flush().unwrap();

        // 更新位置
        for (i, pos) in positions.iter_mut().enumerate() {
            *pos += directions[i];
            if *pos > wave_chars.len() as i32 / 2 || *pos < -(wave_chars.len() as i32) / 2 {
                directions[i] = -directions[i];
            }
        }

        thread::sleep(Duration::from_millis(120));
    }
}

fn retry_animation() {
    hide_cursor();
    let wave_blocks = "▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▂▃▄▅▆▇█▇▆▅▄▃▂▁";
    let wave_chars: Vec<char> = wave_blocks.chars().collect();
    let mut positions = vec![12i32, 12, 12];
    let directions = vec![-1i32, 1, -1];

    for counter in 0..80 {
        let mut core_line = String::new();

        for i in 0..wave_chars.len() {
            let mut max_height = 0;
            for &pos in &positions {
                let distance = ((i as i32 - pos + wave_chars.len() as i32) % wave_chars.len() as i32).abs();
                let distance = if distance > wave_chars.len() as i32 / 2 {
                    wave_chars.len() as i32 - distance
                } else {
                    distance
                };
                let height = wave_chars.len() as i32 - distance;
                if height > max_height {
                    max_height = height;
                }
            }

            let decay_factor = 80 - counter;
            let mut index = ((max_height * decay_factor / 80) * wave_chars.len() as i32 / (wave_chars.len() as i32 + 2)) as usize;
            if index >= wave_chars.len() {
                index = wave_chars.len() - 1;
            }
            core_line.push(wave_chars[index]);
        }

        let remaining = 4 - counter / 20;
        let dots = ".".repeat(((counter % 16) / 4) as usize);
        print!("\r\x1b[K Ebbing:{}[{}]{} 等待 {} 秒后重试{}",
               CYAN_BG, core_line, RESET, remaining, dots);
        io::stdout().flush().unwrap();

        // 更新位置
        for (i, pos) in positions.iter_mut().enumerate() {
            *pos += directions[i] * (80 - counter) / 20;
            *pos = (*pos + wave_chars.len() as i32) % wave_chars.len() as i32;
        }

        thread::sleep(Duration::from_millis(50));
    }
    show_cursor();
}

fn real_progress_bar(url: &str, filename: &str, download_dir: &str) -> Result<(), String> {
    let file_path = Path::new(download_dir).join(filename);
    let file_path_str = file_path.to_str().ok_or("无效文件路径")?.to_string();
    let current_progress = Arc::new(Mutex::new(0u32));

    // Ctrl+C处理
    let current_progress_clone = current_progress.clone();
    let file_path_clone = file_path_str.clone();
    ctrlc::set_handler(move || {
        let progress = *current_progress_clone.lock().unwrap();
        let color_wd = if progress <= 3000 {
            format!("{}{}", RED_WD, ITALIC)
        } else if progress <= 7000 {
            format!("{}{}", YELLOW_WD, ITALIC)
        } else {
            format!("{}{}", GREEN_WD, ITALIC)
        };
        let percent = progress as f64 / 100.0;
        print!("\r\x1b[K{}{} [!]{} {}用户中断{} {}{}(进度:{:.2}%){}\n",
               RED_WD, BLINK, RESET, BOLD, RESET, LB, color_wd, percent, RESET);
        show_cursor();
        let _ = fs::remove_file(&file_path_clone);
        std::process::exit(1);
    }).map_err(|e| format!("设置Ctrl-C处理程序失败: {}", e))?;

    hide_cursor();

    for attempt in 1..=MAX_RETRY {
        *current_progress.lock().unwrap() = 0;

        let client = Client::new();
        let mut response = match client.get(url).send() {
            Ok(r) => r,
            Err(e) => {
                update_progress("下载失败", 0);
                if attempt < MAX_RETRY {
                    real_progress_bar_retry_animation();
                    continue;
                } else {
                    return Err(format!("请求失败: {}", e));
                }
            }
        };

        let total_size = response.content_length().unwrap_or(0);
        let mut file = match fs::File::create(&file_path_str) {
            Ok(f) => f,
            Err(e) => {
                update_progress("创建文件失败", 0);
                if attempt < MAX_RETRY {
                    real_progress_bar_retry_animation();
                    continue;
                } else {
                    return Err(format!("创建文件失败: {}", e));
                }
            }
        };

        let mut downloaded = 0u64;
        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = match response.read(&mut buffer) {
                Ok(0) => break, // 下载完成
                Ok(n) => n,
                Err(e) => {
                    update_progress("读取数据失败", 0);
                    if attempt < MAX_RETRY {
                        real_progress_bar_retry_animation();
                        break;
                    } else {
                        return Err(format!("读取数据失败: {}", e));
                    }
                }
            };

            if let Err(e) = file.write_all(&buffer[..bytes_read]) {
                update_progress("写入文件失败", 0);
                if attempt < MAX_RETRY {
                    real_progress_bar_retry_animation();
                    break;
                } else {
                    return Err(format!("写入文件失败: {}", e));
                }
            }

            downloaded += bytes_read as u64;

            // 更新进度
            let progress = if total_size > 0 {
                (downloaded as f64 / total_size as f64 * 10000.0) as u32
            } else {
                0
            };

            *current_progress.lock().unwrap() = progress;
            update_progress(&format!("尝试下载(第 {} 次)", attempt), progress);
        }

        // 检查下载是否完整
        if total_size > 0 && downloaded < total_size {
            update_progress("下载不完整", 0);
            if attempt < MAX_RETRY {
                real_progress_bar_retry_animation();
                continue;
            } else {
                return Err("下载不完整".into());
            }
        }

        // 下载成功
        let filled_bar = format!("[{}]", "#".repeat(29));
        print!("\r Loading:{}{}{} 100.00% 下载完成\x1b[K\n", GREEN_BG, filled_bar, RESET);
        show_cursor();
        return Ok(());
    }

    println!("\n达到最大重试次数,下载失败");
    let _ = fs::remove_file(&file_path_str);
    show_cursor();
    Err("下载失败".into())
}

fn update_progress(message: &str, current_progress: u32) {
    let current_progress = std::cmp::min(current_progress, 10000);
    let percent = current_progress as f64 / 100.0;

    let color_bg = if current_progress <= 3000 {
        RED_BG
    } else if current_progress <= 7000 {
        YELLOW_BG
    } else {
        GREEN_BG
    };

    let filled = (current_progress * 29 + 5000) / 10000;
    let bar = format!("[{:<29}]", "#".repeat(filled as usize));

    print!("\r Loading:{}{}{} {:6.2}% {}\x1b[K", color_bg, bar, RESET, percent, message);
    io::stdout().flush().unwrap();
}

fn download_file(url: &str, file_path: &str) -> Result<(), String> {
    let client = Client::new();
    let mut response = client.get(url)
        .send()
        .map_err(|e| format!("请求失败: {}", e))?;

    let mut file = fs::File::create(file_path)
        .map_err(|e| format!("创建文件失败: {}", e))?;

    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = response.read(&mut buffer)
            .map_err(|e| format!("读取数据失败: {}", e))?;

        if bytes_read == 0 {
            break; // 下载完成
        }

        file.write_all(&buffer[..bytes_read])
            .map_err(|e| format!("写入文件失败: {}", e))?;
    }

    Ok(())
}

fn real_progress_bar_retry_animation() {
    hide_cursor();
    let delay = Duration::from_millis(50);
    let total = 5 * 20; // 5秒 * 20次/秒 = 100次

    for counter in 0..total {
        let remaining_sec = 5 - counter / 20;
        let dot_phase = (counter / 8) % 4;
        let dots = ".".repeat(dot_phase as usize);
        update_progress(&format!("等待 {} 秒后重试{}", remaining_sec, dots), 0);
        thread::sleep(delay);
    }
    print!("\r\x1b[K"); // 清除行内容
    show_cursor();
}
