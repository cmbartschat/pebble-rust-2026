use core::mem::swap;

use alloc::boxed::Box;

pub type EffectCallback = Box<dyn FnMut() -> EffectCleanup>;
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

pub enum Effect {
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
