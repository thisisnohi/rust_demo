use std::path::Path;
use std::{env, fs, thread, time::Duration};

#[derive(Debug)]
pub struct AppInfo {
    from_path: String,
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
    AppInfo { from_path: from }
}

fn main() {
    // 读取参数
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let current_dir = env::current_dir().unwrap();
    println!("current_dir:{}", current_dir.display());
    let current_dir = current_dir.to_str().unwrap();

    // 不能获取例行程序所有路径
    // let current_dir = env::current_exe().unwrap();
    // let current_dir = current_dir
    //     .parent()
    //     .unwrap()
    //     .as_os_str()
    //     .to_str()
    //     .unwrap()
    //     .to_string();
    println!("current_dir:{}", current_dir);
    // if !current_dir.contains("Download") {
    //     println!("参数异常");
    //     thread::sleep(Duration::from_secs(10));
    //     exit(1)
    // }
    let path = get_param(&args, &current_dir.to_string());

    // 重命名文件
    rename_file(1, &path.from_path);
    println!("任务结束...Good Luck...");
    thread::sleep(Duration::from_secs(5));
}

// 重命名文件
fn rename_file(lvl: usize, dir: &String) {
    println!("操作目录:{:?},深度:{}", dir, lvl);
    let title = "\t".repeat(lvl - 1);

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
            println!("{} 文件[{:?}]是目录，路过", title, entry.file_name());
            // 子目录不在操作
            continue;
        }
        // 文件处理
        let filename = entry.file_name();
        let filename: String = filename.into_string().unwrap();
        // 获取文件后缀
        let extension = if let Some(ext) = get_file_extension(filename.as_str()) {
            ext
        } else {
            println!("{} 文件[{:?}]无后缀，跳过", title, entry.file_name());
            continue;
        };

        let is_pic = match extension {
            "jpg" | "jpeg" | "png" | "svg" => true,
            _ => false,
        };
        // 判断文件类型
        if !is_pic {
            println!("{} 非图片，暂不处理:{}", title, filename);
            continue;
        }

        // 文件包含[ 无需处理
        if !filename.contains("-") {
            println!("{} 文件名不需要处理:{}", title, filename);
            continue;
        }
        let target_file_name = filename.replace("-", "");

        if target_file_name.len() <= 0 {
            println!("{} 无法生成文件名", title);
            continue;
        }

        let src_file = entry_path;
        let target_file = format!("{}/{}", dir, target_file_name);
        println!("{} 重命名文件[{}]=>[{}]", title, src_file, target_file);
        if src_file.len() > 10 && target_file.len() > 10 {
            fs::rename(src_file, target_file).unwrap();
        }
    }

    /// 获取文件扩展名
    fn get_file_extension(file_path: &str) -> Option<&str> {
        let path = Path::new(file_path);
        path.extension().map(|ext| ext.to_str().unwrap())
    }
}
