use std::io;

use rodio::{Decoder, OutputStream, Sink};

static mut SINK: Option<Sink> = None;
static mut STREAM: Option<OutputStream> = None;

/**
 * 初始化音频播放器
 */
pub fn init() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    unsafe {
        STREAM = Some(_stream);
        SINK = Some(sink);
    }
}

/**
 * 加载音频
 */
pub fn load(url: &str) {
    let response = ureq::get(url).call().unwrap();
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes).unwrap();
    let source = Decoder::new(io::Cursor::new(bytes)).unwrap();
    unsafe {
        SINK.as_mut().unwrap().append(source);
    }
}

/**
 * 播放音频
 */
pub fn play() {
    unsafe {
        SINK.as_mut().unwrap().play();
    }
}

/**
 * 暂停音频
 */
pub fn pause() {
    unsafe {
        SINK.as_mut().unwrap().pause();
    }
}

/**
 * 恢复音频
 */
pub fn resume() {
    unsafe {
        SINK.as_mut().unwrap().play();
    }
}

/**
 * 停止音频
 */
pub fn stop() {
    unsafe {
        SINK.as_mut().unwrap().stop();
    }
}

/**
 * 设置音量
 */
pub fn set_volume(volume: f32) {
    unsafe {
        SINK.as_mut().unwrap().set_volume(volume);
    };
}

/**
 * 获取音量
 */
pub fn get_volume() -> f32 {
    unsafe { SINK.as_mut().unwrap().volume() }
}

/**
 * 获取音频状态
 */
pub fn get_status() -> i32 {
    // 0: 已停止 1: 播放中 2: 已暂停
    unsafe {
        let sink = SINK.as_mut().unwrap();
        if sink.is_paused() {
            2 // Paused
        } else if !sink.empty() {
            1 // Playing
        } else {
            0 // Stopped
        }
    }
}
