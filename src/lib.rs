use jni::objects::{JObject, JString};
use jni::sys::jfloat;
use jni::JNIEnv;

pub mod music_player;

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_init(_: JNIEnv, _: JObject) {
    music_player::init();
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_load(
    mut env: JNIEnv,
    _: JObject,
    url: JString,
) {
    let url: String = env
        .get_string(&url)
        .expect("Couldn't get java string!")
        .into();
    music_player::load(&url);
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_play(_: JNIEnv, _: JObject) {
    music_player::play();
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_pause(_: JNIEnv, _: JObject) {
    music_player::pause();
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_resume(_: JNIEnv, _: JObject) {
    music_player::resume();
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_stop(_: JNIEnv, _: JObject) {
    music_player::stop();
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_getVolume(
    _: JNIEnv,
    _: JObject,
) -> jfloat {
    let volume = music_player::get_volume();
    volume.into()
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_setVolume(
    _: JNIEnv,
    _: JObject,
    volume: f32,
) {
    music_player::set_volume(volume);
}

#[no_mangle]
pub extern "system" fn Java_me_zhenxin_zmusic_music_JniPlayer_getStatus(
    _: JNIEnv,
    _: JObject,
) -> i32 {
    music_player::get_status()
}
