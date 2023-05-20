pub mod monie {
    pub mod auth {
        tonic::include_proto!("authentication");
    }

    pub mod accounts {
        tonic::include_proto!("accounts");
    }

    // pub mod chat {
    //     tonic::include_proto!("chat");
    // }

    pub mod media {
        tonic::include_proto!("media");
    }

    pub mod timeline {
        tonic::include_proto!("timeline");
    }

    pub mod user {
        tonic::include_proto!("user");
    }
}

