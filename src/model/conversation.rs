use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone , Debug)]
pub struct Conversation {
    pub msgs : Vec<Msg> ,
}
impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            msgs : Vec::new() ,
        }
    }


}



#[derive(Serialize, Deserialize, Clone , Debug)]
pub struct Msg {
    pub usr : bool ,
    pub txt : String ,
}