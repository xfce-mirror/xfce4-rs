# xfce4-kbd-private

This crate provides a safe wrapper for the `libxfce4kbd-private`
library, which is a utility library for handling keyboard shortcuts for
Xfce.  The underlying C library is intended to be for private use only
by Xfce components, and its API and ABI are subject to change without
considerations for backward-compatibility.  Do not use this crate.
