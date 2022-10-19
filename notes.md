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

## types.rs
This file contains the types(Structures of data types)
Why did I use Lazy<Address> over Address? I have no technical reason
I saw this pattern in gakonst's opensea-rs
Made everything here pub so I could use them in other files

TODO  I have to make sure each type is ideal at some point
I used u64 for every field that was a number, I'm not sure if this was a good idea
FIXME start_amount and end_amount are Strings, I'll have to work on that later
FIXME recipient should probably be an Address or Lazy<Address>

line 20: types.rs - impl SeaportConfig
I faced issues setting the conduit key to a constanst, so I'm using a function for it instead - Got the idea from asnared :)
I might move this to SeaportConfig

line 61
Using a vec here but would an array be better?
const offer: [OfferItem; 1];

line 79 - enum ProviderOrSigneer
I needed a way to set provider or signer to just two types, thus the enum
