use crate::constant::OrderType;
use crate::constant::ItemType;

pub fn run(){
    struct Person {
        name:  String,
        age: u8,
    }

    struct OfferItem {
        item_type: ItemType,
        token: String,
        identifier_or_criteria: String,
        start_amount: String,
        end_amount: String,
    }

    struct ConsiderationItem {
        item_type: ItemType,
        token: String,
        identifier_or_criteria: String,
        start_amount: String,
        end_amount: String,
        recipient: String,
    };



    struct OrderParameters {
        offerer: String,
        zone: String,
        order_type: OrderType,
        start_time: u64,
        end_time: u64,
        zone_hash: String,
        salt: String,
        // Using a vec here but would an array be better?
        // const xs: [OfferItem; 1];
        offer: Vec<OfferItem>,
        consideration: Vec<ConsiderationItem>,
        total_original_consideration_items: u64,
        conduit_key: String
    }


    fn identify(person: Person) -> ( String, u8) {
        (person.name, person.age)
    }

    let name = String::from("Perelyn");
    let age = 20;
    let perelyn : Person = Person { name :name , age: age};

    println!("{:?}",identify(perelyn));
}


// fn main () {
//     let vectorOfStructs: Vec<YourStruct>; 
// }

// struct YourStruct {
//     // stuff
// }

// export type OrderParameters = {
//     offerer: string;
//     zone: string;
//     orderType: OrderType;
//     startTime: BigNumberish;
//     endTime: BigNumberish;
//     zoneHash: string;
//     salt: string;
//     offer: OfferItem[];
//     consideration: ConsiderationItem[];
//     totalOriginalConsiderationItems: BigNumberish;
//     conduitKey: string;
//   };