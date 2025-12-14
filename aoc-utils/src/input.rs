use std::io::Read;

pub type Result<T> = std::result::Result<T, AocUtilError>;

#[derive(Debug)]
pub enum AocUtilError {
    CookieNotFound,
    Io(std::io::Error),
    Http(reqwest::Error),
}

impl From<std::io::Error> for AocUtilError {
    fn from(io_error: std::io::Error) -> Self {
        Self::Io(io_error)
    }
}

impl From<reqwest::Error> for AocUtilError {
    fn from(http_error: reqwest::Error) -> Self {
        Self::Http(http_error)
    }
}

fn read_env_file() -> Result<()> {
    let env_filepath = std::path::PathBuf::from("../.env");

    if !env_filepath.exists() {
        return Ok(());
    }

    let content = {
        let mut buf = String::new();
        let mut reader = std::io::BufReader::new(std::fs::File::open(&env_filepath)?);
        reader.read_to_string(&mut buf)?;

        buf
    };

    for line in content.lines() {
        let (key, value) = line.split_once("=").ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid key-value pair in .env",
        ))?;
        unsafe { std::env::set_var(key, value) }
    }

    Ok(())
}

fn read_session_cookie() -> Result<String> {
    read_env_file()?;

    let session_cookie =
        std::env::var("SESSION_COOKIE").map_err(|_| AocUtilError::CookieNotFound)?;

    Ok(session_cookie)
}

fn fetch_input(year: usize, day: usize) -> Result<String> {
    let session_cookie = read_session_cookie()?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;

    Ok(response.text()?)
}

fn year_day_to_filepath(year: usize, day: usize) -> std::path::PathBuf {
    let filename = format!("day{:02}.txt", day);
    let input_dir = std::path::PathBuf::from(format!("../input/{}", year));
    input_dir.join(std::path::PathBuf::from(filename))
}

pub fn download_input_file(year: usize, day: usize) -> Result<()> {
    let input_filepath = year_day_to_filepath(year, day);

    if input_filepath.exists() {
        return Ok(());
    }

    let input = fetch_input(year, day)?;

    let input_dir = std::path::PathBuf::from(format!("../input/{}", year));

    std::fs::create_dir_all(&input_dir)?;

    std::fs::write(input_filepath, input)?;

    Ok(())
}

pub fn read_input_file(year: usize, day: usize) -> Result<String> {
    let input_filepath = year_day_to_filepath(year, day);
    std::fs::read_to_string(input_filepath).map_err(AocUtilError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_session_cookie_works() {
        read_session_cookie().unwrap();

        let session_cookie = std::env::var("SESSION_COOKIE").unwrap();
        assert!(!session_cookie.is_empty());
    }

    #[test]
    fn fetch_input_test() {
        let year = 2019;
        let day = 13;
        let input = fetch_input(year, day).unwrap();

        assert!(input.len() > 100);
    }

    #[test]
    fn download_input_file_test() {
        let year = 2019;
        let day = 13;
        let result = download_input_file(year, day);

        assert!(result.is_ok());
    }
}
