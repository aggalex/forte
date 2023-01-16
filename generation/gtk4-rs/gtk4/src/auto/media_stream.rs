// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkMediaStream")]
    pub struct MediaStream(Object<ffi::GtkMediaStream, ffi::GtkMediaStreamClass>) @implements gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_media_stream_get_type(),
    }
}

impl MediaStream {
    pub const NONE: Option<&'static MediaStream> = None;
}

pub trait MediaStreamExt: 'static {
    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_media_stream_ended")]
    fn ended(&self);

    #[doc(alias = "gtk_media_stream_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> i64;

    #[doc(alias = "gtk_media_stream_get_ended")]
    #[doc(alias = "get_ended")]
    fn is_ended(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_error")]
    #[doc(alias = "get_error")]
    fn error(&self) -> Option<glib::Error>;

    #[doc(alias = "gtk_media_stream_get_loop")]
    #[doc(alias = "get_loop")]
    fn is_loop(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_muted")]
    #[doc(alias = "get_muted")]
    fn is_muted(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_playing")]
    #[doc(alias = "get_playing")]
    fn is_playing(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_timestamp")]
    #[doc(alias = "get_timestamp")]
    fn timestamp(&self) -> i64;

    #[doc(alias = "gtk_media_stream_get_volume")]
    #[doc(alias = "get_volume")]
    fn volume(&self) -> f64;

    #[doc(alias = "gtk_media_stream_has_audio")]
    fn has_audio(&self) -> bool;

    #[doc(alias = "gtk_media_stream_has_video")]
    fn has_video(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_prepared")]
    fn is_prepared(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_seekable")]
    fn is_seekable(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_seeking")]
    fn is_seeking(&self) -> bool;

    #[doc(alias = "gtk_media_stream_pause")]
    fn pause(&self);

    #[doc(alias = "gtk_media_stream_play")]
    fn play(&self);

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_media_stream_prepared")]
    fn prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64);

    #[doc(alias = "gtk_media_stream_realize")]
    fn realize(&self, surface: &impl IsA<gdk::Surface>);

    #[doc(alias = "gtk_media_stream_seek")]
    fn seek(&self, timestamp: i64);

    #[doc(alias = "gtk_media_stream_seek_failed")]
    fn seek_failed(&self);

    #[doc(alias = "gtk_media_stream_seek_success")]
    fn seek_success(&self);

    #[doc(alias = "gtk_media_stream_set_loop")]
    fn set_loop(&self, loop_: bool);

    #[doc(alias = "gtk_media_stream_set_muted")]
    fn set_muted(&self, muted: bool);

    #[doc(alias = "gtk_media_stream_set_playing")]
    fn set_playing(&self, playing: bool);

    #[doc(alias = "gtk_media_stream_set_volume")]
    fn set_volume(&self, volume: f64);

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_media_stream_stream_ended")]
    fn stream_ended(&self);

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_media_stream_stream_prepared")]
    fn stream_prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64);

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_media_stream_stream_unprepared")]
    fn stream_unprepared(&self);

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_media_stream_unprepared")]
    fn unprepared(&self);

    #[doc(alias = "gtk_media_stream_unrealize")]
    fn unrealize(&self, surface: &impl IsA<gdk::Surface>);

    #[doc(alias = "gtk_media_stream_update")]
    fn update(&self, timestamp: i64);

    fn set_prepared(&self, prepared: bool);

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ended")]
    fn connect_ended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "error")]
    fn connect_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-audio")]
    fn connect_has_audio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-video")]
    fn connect_has_video_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "loop")]
    fn connect_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "muted")]
    fn connect_muted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "playing")]
    fn connect_playing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "prepared")]
    fn connect_prepared_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "seekable")]
    fn connect_seekable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "seeking")]
    fn connect_seeking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timestamp")]
    fn connect_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "volume")]
    fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MediaStream>> MediaStreamExt for O {
    #[allow(deprecated)]
    fn ended(&self) {
        unsafe {
            ffi::gtk_media_stream_ended(self.as_ref().to_glib_none().0);
        }
    }

    fn duration(&self) -> i64 {
        unsafe { ffi::gtk_media_stream_get_duration(self.as_ref().to_glib_none().0) }
    }

    fn is_ended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_ended(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_none(ffi::gtk_media_stream_get_error(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_loop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_loop(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_muted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_muted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_playing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_playing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timestamp(&self) -> i64 {
        unsafe { ffi::gtk_media_stream_get_timestamp(self.as_ref().to_glib_none().0) }
    }

    fn volume(&self) -> f64 {
        unsafe { ffi::gtk_media_stream_get_volume(self.as_ref().to_glib_none().0) }
    }

    fn has_audio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_has_audio(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_video(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_has_video(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_prepared(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_prepared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_seekable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_seeking(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_seeking(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pause(&self) {
        unsafe {
            ffi::gtk_media_stream_pause(self.as_ref().to_glib_none().0);
        }
    }

    fn play(&self) {
        unsafe {
            ffi::gtk_media_stream_play(self.as_ref().to_glib_none().0);
        }
    }

    #[allow(deprecated)]
    fn prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64) {
        unsafe {
            ffi::gtk_media_stream_prepared(
                self.as_ref().to_glib_none().0,
                has_audio.into_glib(),
                has_video.into_glib(),
                seekable.into_glib(),
                duration,
            );
        }
    }

    fn realize(&self, surface: &impl IsA<gdk::Surface>) {
        unsafe {
            ffi::gtk_media_stream_realize(
                self.as_ref().to_glib_none().0,
                surface.as_ref().to_glib_none().0,
            );
        }
    }

    fn seek(&self, timestamp: i64) {
        unsafe {
            ffi::gtk_media_stream_seek(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn seek_failed(&self) {
        unsafe {
            ffi::gtk_media_stream_seek_failed(self.as_ref().to_glib_none().0);
        }
    }

    fn seek_success(&self) {
        unsafe {
            ffi::gtk_media_stream_seek_success(self.as_ref().to_glib_none().0);
        }
    }

    fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gtk_media_stream_set_loop(self.as_ref().to_glib_none().0, loop_.into_glib());
        }
    }

    fn set_muted(&self, muted: bool) {
        unsafe {
            ffi::gtk_media_stream_set_muted(self.as_ref().to_glib_none().0, muted.into_glib());
        }
    }

    fn set_playing(&self, playing: bool) {
        unsafe {
            ffi::gtk_media_stream_set_playing(self.as_ref().to_glib_none().0, playing.into_glib());
        }
    }

    fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::gtk_media_stream_set_volume(self.as_ref().to_glib_none().0, volume);
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    fn stream_ended(&self) {
        unsafe {
            ffi::gtk_media_stream_stream_ended(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    fn stream_prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64) {
        unsafe {
            ffi::gtk_media_stream_stream_prepared(
                self.as_ref().to_glib_none().0,
                has_audio.into_glib(),
                has_video.into_glib(),
                seekable.into_glib(),
                duration,
            );
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    fn stream_unprepared(&self) {
        unsafe {
            ffi::gtk_media_stream_stream_unprepared(self.as_ref().to_glib_none().0);
        }
    }

    #[allow(deprecated)]
    fn unprepared(&self) {
        unsafe {
            ffi::gtk_media_stream_unprepared(self.as_ref().to_glib_none().0);
        }
    }

    fn unrealize(&self, surface: &impl IsA<gdk::Surface>) {
        unsafe {
            ffi::gtk_media_stream_unrealize(
                self.as_ref().to_glib_none().0,
                surface.as_ref().to_glib_none().0,
            );
        }
    }

    fn update(&self, timestamp: i64) {
        unsafe {
            ffi::gtk_media_stream_update(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn set_prepared(&self, prepared: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "prepared", &prepared)
    }

    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ended_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ended\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ended_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_error_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_error_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_has_audio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_audio_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-audio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_audio_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_has_video_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_video_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-video\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_video_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loop_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::loop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_muted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_muted_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::muted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_muted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_playing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_playing_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::playing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_playing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_prepared_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_prepared_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::prepared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_prepared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_seekable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seekable_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seekable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seekable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_seeking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seeking_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seeking\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seeking_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_trampoline<
            P: IsA<MediaStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<P: IsA<MediaStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MediaStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MediaStream")
    }
}
