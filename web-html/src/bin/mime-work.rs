use mime::{Mime, APPLICATION_OCTET_STREAM};

fn main() {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "Mime for {:?} use default value {:?}",
        invalid_mime_type, default_mime
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} was parsed as {:?}",
        valid_mime_type, parsed_mime
    );

    let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];

    for file in filenames {
        let mime = {
            let file = &file;
            let parts = &file.to_owned().split(".").collect::<Vec<&str>>();
            let res = match parts.last() {
                Some(&"png") => mime::IMAGE_PNG,
                Some(&"jpg") => mime::IMAGE_JPEG,
                Some(&"json") => mime::APPLICATION_JSON,
                _ => mime::TEXT_PLAIN,
            };
            res
        };

        println!("MIME for {}: {}", file, mime);
    }
}
