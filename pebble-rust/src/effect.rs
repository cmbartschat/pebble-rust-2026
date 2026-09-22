use core::mem::swap;

use alloc::boxed::Box;

/// The callback for starting an effect.
/// The return value of such a callback is an [`EffectCleanup`] to call when the effect is ended.
///
/// Various functions take an [`EffectCallback`] as a "fancy handler function".
/// However, the effect system additionally guarantees the following:
/// - Every call of the effect callback is paired with exactly one call to the cleanup function it returned.
///   This also means you can return different cleanups depending on internal logic.
/// - The effect callback is called exactly one time once the effect starts.
/// - The cleanup function is called exactly one time once the effect ends.
///   After that the effect returns to its initial state and will call the original callback again once the effect is triggered again.
/// - Replacing an effect while the cleanup function hasn’t yet been invoked will immediately invoke it, to guarantee the above.
/// - If the underlying effect source (e.g. a window load) triggers multiple times without the cleanup source (e.g. a window unload) also triggering,
///   the effect callback is not called multiple times, but only once.
///   The same goes for multiple cleanup source triggers in series.
// TODO: These guarantees do not hold when the effect is dropped.
//       Simply implementing Drop for Effect is not possible due to dropcheck shenanigans.
pub type EffectCallback = Box<dyn FnMut() -> EffectCleanup>;
/// The callback for ending or cleaning up an effect.
/// See [`EffectCallback`] for details on the effect lifecycle.
pub type EffectCleanup = Box<dyn FnOnce()>;

pub(crate) struct PendingEffect {
    callback: EffectCallback,
}

impl PendingEffect {
    pub fn new(callback: EffectCallback) -> Self {
        Self { callback }
    }

    pub fn mount(mut self) -> MountedEffect {
        let cleanup = Box::new((self.callback)());
        MountedEffect {
            callback: self.callback,
            cleanup,
        }
    }
}

pub(crate) struct MountedEffect {
    callback: EffectCallback,
    cleanup: EffectCleanup,
}

impl MountedEffect {
    pub fn unmount(self) -> PendingEffect {
        (self.cleanup)();
        PendingEffect {
            callback: self.callback,
        }
    }
}

pub(crate) enum Effect {
    Pending(PendingEffect),
    Mounted(MountedEffect),
    NoneMounted,
    None,
}

impl Effect {
    pub(crate) fn unmount(&mut self) {
        let mut taken = Self::None;
        swap(self, &mut taken);
        *self = taken.unmount_owned();
    }

    fn unmount_owned(self) -> Self {
        match self {
            Self::Mounted(mounted) => Effect::Pending(mounted.unmount()),
            Self::NoneMounted => Self::None,
            Self::Pending(..) => self,
            Self::None => self,
        }
    }

    pub(crate) fn mount(&mut self) {
        let mut taken = Self::None;
        swap(self, &mut taken);
        *self = taken.mount_owned();
    }

    fn mount_owned(self) -> Self {
        match self {
            Self::Mounted(..) => self,
            Self::NoneMounted => self,
            Self::Pending(u) => Effect::Mounted(u.mount()),
            Self::None => Self::NoneMounted,
        }
    }

    pub(crate) fn set_callback(&mut self, callback: EffectCallback) {
        let mut taken = Self::Pending(PendingEffect::new(callback));
        swap(self, &mut taken);

        let was_mounted = match taken {
            Self::Mounted(pending_effect) => {
                pending_effect.unmount();
                true
            }
            Self::NoneMounted => true,
            Self::None | Self::Pending(..) => false,
        };

        if was_mounted {
            self.mount();
        }
    }
}
