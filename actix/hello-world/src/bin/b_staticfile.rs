use actix_files as fs;
use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::middleware::Logger;
use actix_web::{get, Error, HttpRequest, Result};
use log::info;
use std::path::PathBuf;

/// # 静态文件
/// 可以自定义路径和 NamedFile 来提供静态文件。要匹配路径尾，我们可以使用 [.*] 正则表达式。

async fn index(req: HttpRequest) -> Result<NamedFile> {
    info!("index...");

    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    info!("path: {:?}", path);

    Ok(NamedFile::open(path)?)
}

#[get("/{filename:.*}")]
async fn open_file(req: HttpRequest) -> Result<NamedFile, Error> {
    info!("open_file...");
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    info!("path: {:?}", path);
    let file = NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    // 设置日志参数
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    // 日期初始化
    env_logger::init();

    HttpServer::new(|| {
        // 加载日志
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // http://127.0.0.1:8080/open/Cargo.toml
            .service(web::scope("/open").route("/{filename:.*}", web::get().to(index)))
            // http://127.0.0.1:8080/open_file/Cargo.toml
            .service(web::scope("/open_file").service(open_file))
            .service(
                // 无法显示
                web::scope("/static").service(fs::Files::new("/static", ".").show_files_listing()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
