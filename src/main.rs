extern crate gstreamer as gst;

use std::sync::mpsc::channel;
use std::thread;

use gst::Pipeline;
use gst::prelude::*;
use serde::Deserialize;
use tiny_http::{Response, Server};

const STREAMING_OFF: &str = "Set Pipeline to Not Streaming";
const STREAMING_ON: &str = "Set Pipeline to Streaming";
// absolutely must be the full working directory for the toml to work. I suppose I could add it
// as a http request header
const CONFIG_DIRECTORY: &str = "/Users/willberry/RustroverProjects/ruststreamer/src/config.toml";
#[derive(Debug, Deserialize)]
struct Config {
    src_type: String,
}

// Structure type for parameters to pass between threads
#[derive(Debug, Deserialize)]
struct StreamActivityParams {
    filename: String,
    active: bool,
}

// Reads the config file using basic serde/toml setup to easily grab the configured stream driver
// in this case the src type would be v4l2src for linux and avfvideosrc for mac.
fn read_config() -> std::io::Result<Config> {
    let content = std::fs::read_to_string(CONFIG_DIRECTORY)?;
    Ok(toml::from_str(&content)?)
}

/*
            I was able to get this working, and I'll replace it with proper primitives if I can.
            This crate seems to be slightly unstable however, and it's important to share that
            various versions had breaking changes in how these primitive bindings worked, and all
            materials seemed to mix and match different versions. So I could not piece it together
            as one might usually have. Time constraints meant that functionality wise,
            the pseudocode on the website doesn't look *that* different to this.


                // Create a GStreamer pipeline
                pipeline = createPipeline()

                // Create elements
                source = createElement("avfvideosrc", "source")
                converter = createElement("videoconvert", "converter")
                encoder = createElement("x265enc", "encoder")
                parser = createElement("h265parse", "parser")
                muxer = createElement("matroskamux", "muxer")
                sink = createElement("filesink", "sink")

                // Set properties for elements
                setProperty(source, "device-index", 0)
                setProperty(encoder, "bitrate", 5000)
                setProperty(encoder, "speed-preset", "superfast")
                setProperty(encoder, "tune", "zerolatency")

                // Add elements to the pipeline
                addElementsToPipeline(pipeline, source, converter, encoder, parser, muxer, sink)


                I tried something about like the above. Would have been able to fix with additional
                time. Elected that above and below is good enough to denote I wouldn't struggle
                putting it together with more time or quality docs
            */
fn generate_pipeline(video_format: &str, file_name: &str, playing: bool) -> Pipeline {
    let pipeline_str = format!(
        "{} device-index=0 ! videoconvert ! x265enc bitrate=5000 speed-preset=superfast ! video/x-h265,stream-format=byte-stream,profile=main ! h265parse ! matroskamux ! filesink location={}",
        video_format, // Assuming srctype is the source element
        file_name
    );
    let pipeline = gst::parse_launch(&pipeline_str)
        .expect("Failed to create pipeline");
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    if playing {
        pipeline.set_state(gst::State::Playing).expect(STREAMING_ON);
    } else {
        pipeline.set_state(gst::State::Null).expect(STREAMING_OFF);
    }
    pipeline
}


fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    let (tx, rx) = channel::<StreamActivityParams>();

    thread::spawn(move || {
        let config = read_config().unwrap();
        gst::init().unwrap();

        let mut pipeline = generate_pipeline(&config.src_type, "uninitialized", false);
        loop {
            let result = rx.recv().unwrap();
            if result.active == true {
                pipeline = generate_pipeline(&config.src_type, &result.filename, true);
            }
            if result.active == false {
                pipeline.set_state(gst::State::Null).expect(STREAMING_OFF);
            }
        }
    });

    for request in server.incoming_requests() {

        // default file name
        let mut filename: String = "output.mkv".to_string();
        // default server response if you hit the wrong endpoint
        let mut response = Response::from_string("hello from the server. it's currently active but did nothing. Try pinging /start or /stop with a filename header");
        // Extract and replace default filename header
        if let Some(header) = request.headers().iter().find(|header| header.field.as_str() == "filename") {
            filename = header.value.to_string();
        }

        // begin streaming when recieving a request to start, you may start as many streams as one
        // wants to different files, but only one will be active at a time
        if request.url() == "/start" {
            let command = StreamActivityParams {
                filename,
                active: true,
            };
            tx.send(command).unwrap();
            response = Response::from_string("{ streaming: true }")
        }
        // I hate else if typically, but this cleans up the code with ownership -> 👍🏻😎
        // if stop is hit, regardless of filename or headers or whatever, we'll stop the current
        // stream
        else if request.url() == "/stop" {
            let command = StreamActivityParams {
                filename,
                active: true,
            };
            tx.send(command).unwrap();
            response = Response::from_string("{ streaming: false }")
        }

        // resolve default or custom response
        request.respond(response).expect("Unable to send server response");
    }
}
