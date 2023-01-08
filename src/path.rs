#[napi]
mod path {
  use netidx::path::Path;

  #[napi]
  pub fn root() -> String {
    String::from("/")
  }

  #[napi]
  pub fn is_absolute(path: String) -> bool {
    Path::is_absolute(&path)
  }

  #[napi]
  pub fn is_parent(parent: String, other: String) -> bool {
    Path::is_parent(&parent, &other)
  }

  #[napi]
  pub fn is_immediate_parent(parent: String, other: String) -> bool {
    Path::is_immediate_parent(&parent, &other)
  }

  #[napi]
  pub fn strip_prefix(prefix: String, path: String) -> Option<String> {
    Path::strip_prefix(&prefix, &path).map(|s| String::from(s))
  }

  #[napi]
  pub fn lcp(path0: String, path1: String) -> String {
    String::from(Path::lcp(&path0, &path1))
  }

  #[napi]
  pub fn escape(path: String) -> String {
    String::from(Path::escape(&path))
  }

  #[napi]
  pub fn unescape(path: String) -> String {
    String::from(Path::unescape(&path))
  }

  #[napi]
  pub fn append(path: String, other: String) -> String {
    Path::from(&path).append(&other).to_string()
  }

  #[napi]
  pub fn parts(path: String) -> Vec<String> {
    Path::parts(&path).map(|s| String::from(s)).collect()
  }

  #[napi]
  pub fn dirnames(path: String) -> Vec<String> {
    Path::dirnames(&path).map(|d| String::from(d)).collect()
  }

  #[napi]
  pub fn levels(path: String) -> u32 {
    Path::levels(&path) as u32
  }

  #[napi]
  pub fn dirname(path: String) -> Option<String> {
    Path::dirname(&path).map(|s| String::from(s))
  }

  #[napi]
  pub fn dirname_with_sep(path: String) -> Option<String> {
    Path::dirname_with_sep(&path).map(|s| String::from(s))
  }

  #[napi]
  pub fn basename(path: String) -> Option<String> {
    Path::basename(&path).map(|s| String::from(s))
  }

  #[napi]
  pub fn rfind_sep(path: String) -> Option<u32> {
    Path::rfind_sep(&path).map(|u| u as u32)
  }

  #[napi]
  pub fn find_sep(path: String) -> Option<u32> {
    Path::find_sep(&path).map(|u| u as u32)
  }
}
