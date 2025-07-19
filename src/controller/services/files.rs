use crate::{
    controller::responses::{respond_error, ContentTypes, RequestHandler, Response, Statuses},
    stream::stream_write_string,
};
use std::{
    fs::{self, File}, io::Write, path::Path
};

static DEFAULT_FOLDER_PATH: &str = "/tmp/data/codecrafters.io/http-server-tester/";

pub fn get(handle: RequestHandler) {
    let RequestHandler {
        stream,
        req,
        prefix,
    } = handle;

    // Debugging
    // let folder_path = Path::new(DEFAULT_FOLDER_PATH);
    // let paths = fs::read_dir(folder_path).unwrap();
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    let file = req.request_line.get_sub_path(&prefix);
    let file_path = Path::join(Path::new(DEFAULT_FOLDER_PATH), Path::new(&file));

    let body = fs::read_to_string(&file_path);

    match body {
        Ok(body) => {
            let content_length = Some(fs::metadata(file_path).expect("file went missing").len());

            let res = Response {
                content_type: Some(ContentTypes::ApplicationOctetStream),
                content_length,
                body: Some(body),
                ..Default::default()
            };

            stream_write_string(stream, &res.to_string());
        }
        Err(_) => {
            respond_error(stream);
        }
    }
}

pub fn post(handle: RequestHandler) {
    let RequestHandler {
        stream,
        req,
        prefix,
    } = handle;

    let file = req.request_line.get_sub_path(&prefix);
    let file_path = Path::join(Path::new(DEFAULT_FOLDER_PATH), Path::new(&file));
    let mut f = File::create(file_path).expect("Could not create file");
    let _ = f.write_all(req.body.as_bytes());
    let res = Response {
        code: 201,
        status: Statuses::Created,
        ..Default::default()
    };

    stream_write_string(stream, &res.to_string());
}
