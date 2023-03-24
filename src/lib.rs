pub mod db;

pub mod hello {
    tonic::include_proto!("hello");
}

pub mod ondo_remote {
    tonic::include_proto!("ondo_remote");
}
