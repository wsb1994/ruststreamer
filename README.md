Webcam Streaming with GStreamer
Note: Your webcam must be enabled and your laptop open for this code to work. This code will not function if a webcam is not connected.

Prerequisites
Operating System: This code was tested only on macOS.
GStreamer Installation: Ensure GStreamer is installed and tested using the provided script below.


Setup Instructions

1. Configure the Source Type:

Navigate to src/config.toml and set your src_type:

For macOS, use avfvideosrc.
For the original script modified to work on macOS, use v4l2src.

2. Set the Configuration Directory:

Set the CONFIG_DIRECTORY constant to the full path of your config.toml file. Example:

CONFIG_DIRECTORY = "/Users/yourusername/YourProject/src/config.toml"

3. Test GStreamer Installation:

Use the script below to ensure GStreamer is properly installed and functioning. If the script does not run, the code is unlikely to work:

sh
Copy code
GST_DEBUG=2 gst-launch-1.0 -e v4l2src device=/dev/video0 ! videoconvert ! x265enc bitrate=5000 speed-preset=superfast ! "video/x-h265,stream-format=(string)byte-stream,profile=(string)main" ! h265parse ! matroskamux ! filesink location=output.mkv

4. navigate to the root directory of the project and run cargo build. It should build for sure on macosx. If not it may be an issue with gstreamer or the plugin you're using to stream from the webcam.

5. cargo run 

6. Ping localhost:8000/start to start the stream either on the default output.mkv, or provide a header of the format {"filename": "different.mkv"} to record to a different file. You may switch which file you are recording to at any time using this same command. only one stream can be active at a time.
7. Ping localhost:8000/stop and it will stop the stream regardless of which stream is active.
