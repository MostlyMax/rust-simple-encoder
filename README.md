# Rust Simple Encoder
Originally a project written in C to learn basic threading, I remade it in Rust using all
of its fancy bells and whistles.

Surprisingly, I found that the simple (maybe too simple?) version I made in C wasn't easily changed
into Rust as the compiler would yell at me "you really shouldn't be sharing memory like this!". I'm
certain that someone with more Rust experience than me could navigate through the lifetimes and 
smartpointers to make it work, but alas.

Instead, this version uses the fancy tool Rayon to handle the multithreading for me. One less wheel 
reinvented and here we are. The Rust Simple Encoder.

