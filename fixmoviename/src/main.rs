use std::{env, fs, process::exit, thread, time::Duration};

#[derive(Debug)]
pub struct AppInfo {
    from_path: String,
    to_path: String,
}

/**
 * 获取参数
 */
fn get_param(args: &Vec<String>, default_path: &String) -> AppInfo {
    let from: String = if args.len() > 1 {
        args[1].clone()
    } else {
        default_path.clone()
    };
    let to: String = if args.len() > 2 {
        args[2].clone()
    } else {
        default_path.clone()
    };
    AppInfo {
        from_path: from,
        to_path: to,
    }
}

fn main() {
    // 读取参数
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let current_dir = env::current_exe().unwrap();
    let current_dir = current_dir
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    println!("current_dir:{}", current_dir);
    if !current_dir.contains("Download") {
        println!("参数异常");
        thread::sleep(Duration::from_secs(10));
        exit(1)
    }
    let path = get_param(&args, &current_dir);

    // 重命名文件
    rename_file(1, &path.from_path, &path.to_path);
    println!("任务结束...Good Luck...");
    thread::sleep(Duration::from_secs(5));
}

// 重命名文件
fn rename_file(lvl: usize, dir: &String, target_path: &String) {
    println!("操作目录:{:?},深度:{}", dir, lvl);
    let title = "\t".repeat(lvl - 1);
    if lvl > 2 {
        println!("{} 文件深度超过2,不再处理", title);
        return;
    }

    let file_list: fs::ReadDir = fs::read_dir(dir).expect(format!("操作文件异常:{}", dir).as_str());
    // 遍历
    for entry in file_list.into_iter() {
        let entry = entry.unwrap();
        println!("{} 文件[{:?}]", title, entry.file_name());
        let entry_path = entry.path().to_str().unwrap().to_string();
        // 文件属性
        let metadata = fs::metadata(&entry_path).unwrap();
        // println!("meta:{:?}", metadata)
        if metadata.is_dir() {
            // 进入目录内操作
            rename_file(lvl + 1, &entry_path, &target_path);
            continue;
        }
        // 文件处理
        let filename = entry.file_name();
        let filename: String = filename.into_string().unwrap();

        // 判断文件类型
        let video_type = vec![".mp4", ".mov"];
        if !video_type.into_iter().any(|x| filename.ends_with(x)) {
            println!("{} 非视频文件，暂不处理:{}", title, filename);
            continue;
        }

        // 文件包含[ 无需处理
        if !filename.contains("[") {
            println!("{} 文件名不需要处理:{}", title, filename);
            continue;
        }
        let name_array: Vec<&str> = filename.split(&['[', ']'][..]).collect();
        let name_array: Vec<&str> = name_array
            .into_iter()
            .filter(|&x| x.len() != 0 && !(x.contains("www") || x.contains("电影天堂")))
            .collect();
        println!("{} 目标文件名[{:?}]", title, name_array);

        if name_array.len() <= 0 {
            println!("{} 无法生成文件名", title);
            continue;
        }

        let src_file = entry_path;
        let target_file = format!("{}/{}", target_path, name_array.join(""));
        println!("{} 重命名文件[{}]=>[{}]", title, src_file, target_file);
        if src_file.len() > 10 && target_file.len() > 10 {
            fs::rename(src_file, target_file).unwrap();
        }
    }
}
