use std::path::Path;

pub async fn get_game_urls(
    path: String,
) -> Result<Vec<crate::download::Download>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/114.0",
        ),
    );
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_static("keep-alive"),
    );
    headers.insert(
        reqwest::header::CACHE_CONTROL,
        reqwest::header::HeaderValue::from_static("no-cache"),
    );

    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_static("cdn.darkanddarker.com"),
    );

    let res = client
        .get("http://cdn.darkanddarker.com/Dark%20and%20Darker/PatchFileList.txt")
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;
    let body_str = format!("path,sha256,size\n{}", String::from_utf8_lossy(&res));

    let mut files = Vec::new();

    let mut r = csv::Reader::from_reader(body_str.as_bytes());
    for record in r.records() {
        let record = record?;
        let file_path = record[0].to_string();
        let file_name = file_path.split('\\').last().unwrap().to_string();
        let uri = file_path.replace("\\", "/");
        let sha256 = record[1].to_string().to_lowercase();
        let size = record[2].parse::<u64>().unwrap();
        // Remove the first character
        let full_path = Path::new(&path).join(&uri.clone().get(1..).unwrap().to_string());
        files.push(crate::download::Download {
            path: file_path,
            linux_path: full_path.as_os_str().to_str().unwrap().to_string(),
            url: format!(
                "http://cdn.darkanddarker.com/Dark%20and%20Darker/Patch{}",
                uri
            ),
            file_name,
            sha256,
            size,
        });
    }
    Ok(files)
}
