# diLay
This is my attempt to implement a vst plugin in Rust.
I decided to implement a delay, because i don't need to know things like fourier transformation.
At the moment i can use this plugin in ableton live (other DAWs not tested).

the plugin is based on this library:
https://github.com/overdrivenpotato/rust-vst2


#### Goals
    detecting Samplerate
    changing delay time in an effective way
    dry/wet + feedback
    GUI
    
