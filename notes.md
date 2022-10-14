# Notes for each file

## Seaport-rs

This is where most of the work in the project will be done Most of the functions I want to implement will be here but I
have to get the `new` function to work first lol :)
I have no idea why people use the pattern of structs and impls, I don't know what the pattern is called or its
advantages, I swa it off Opensea-rs and some other code-bases

### Struct Seaport

line 14: seaport.rs 
Options are used alot in this function because I wanted to create a situation where the function could be called without a cfg being provided.

line 18: seaport.rs
The idea here is to be able to tell if a config was passed
This line was made with the assumption that If cfg.ascending_amount_fulfillment_buffer is present then a config was provided
cfg.ascending_amount_fulfillment_buffer is not special to this theory, any other field could have been used

line 26: seaport.rs
This is a case when a cfg is not provided
I use SeaportConfig::default()-line 51 has a way to provide a default a base config

line 36: seaport.rs
I made this because I need to create a default config
I found a pattern like in Opensea-rs and thought "Yeah, I could use that"

line 44: seaport.rs
Most to the value have Some covering it because I made them Options