pub mod db;
mod callback_iterator;

pub mod hello { 
    tonic::include_proto!("hello"); 
}

pub mod ondo_remote { 
    tonic::include_proto!("ondo_remote"); 
}
