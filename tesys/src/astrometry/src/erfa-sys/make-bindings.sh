#!/bin/bash
bindgen wrapper.h --whitelist-function 'era.*' --whitelist-var "ERFA_.*" > src/bindings.rs