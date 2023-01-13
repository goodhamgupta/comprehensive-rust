#![allow(dead_code, unused_variables)]

// My implementation: For some reason, char comparison using == doesn't work
// For eg: / == / returns false
// pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
//     let mut prefix_chars = prefix.chars();
//     let mut request_chars = request_path.chars();
//     let prefix_len = prefix.len();
//     let request_len = request_path.len();
//     if prefix_len == request_len {
//         request_path.starts_with(prefix)
//     } else if prefix_len > request_len {
//         false
//     } else {
//         let mut path_idx: usize = 0;
//         let mut mismatch = false;
//         for (idx, c) in prefix.chars().enumerate() {
//             println!(
//                 "prefix: {} req: {}",
//                 c,
//                 request_chars.nth(path_idx).unwrap()
//             );
//             println!("{}", c == '/');
//             if c == '*' {
//                 while ((path_idx + 1) < request_len)
//                     && (request_chars.nth(path_idx + 1).unwrap() != '/')
//                 {
//                     path_idx += 1
//                 }
//             } else if c == request_chars.nth(path_idx).unwrap() {
//                 path_idx += 1;
//                 continue;
//             } else {
//                 println!("here");
//                 mismatch = true;
//                 break;
//             }
//         }
//         if mismatch {
//             false
//         } else {
//             true
//         }
//     }
// }
//

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefixes = prefix.split("/");
    let request_paths = request_path
        .split("/")
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix, request_path) in prefixes.zip(request_paths) {
        match request_path {
            Some(request_path) => {
                if (prefix != "*") && (prefix != request_path) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
