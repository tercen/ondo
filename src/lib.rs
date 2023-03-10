pub mod db;
mod callback_iterator;

pub mod hello { 
    tonic::include_proto!("hello"); 
}

pub mod remote { 
    tonic::include_proto!("remote"); 
}
