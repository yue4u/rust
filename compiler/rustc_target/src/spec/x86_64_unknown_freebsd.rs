use crate::spec::{LinkerFlavor, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = super::freebsd_base::opts();
    base.cpu = "x86-64".to_string();
    base.max_atomic_width = Some(64);
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
    // don't use probe-stack=inline-asm until rust-lang/rust#83139 is resolved.
    base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "x86_64-unknown-freebsd".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .to_string(),
        arch: "x86_64".to_string(),
        options: base,
    }
}
