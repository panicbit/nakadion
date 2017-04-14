error_chain! { 

    types {
        ClientError, ClientErrorKind, ClientResultExt, ClientResult;
    }  

    links {
        Token(::tokenerrors::TokenError, ::tokenerrors::TokenErrorKind);
    }

    errors {
        Connection(t: String) {
            description("There was a problem with the connection.")
            display("Connection error: '{}'", t)
        }
        Request(t: String) {
            description("Invalid request to Nakadi.")
            display("The request was invalid: '{}'", t)
        }
        NoSubscription(t: String) {
            description("The subscription was not known to Nakadi.")
            display("The subscription was not known: '{}'", t)
        }
        Forbidden(t: String) {
            description("Access is forbidden for the client or event type")
            display("Access is forbidden for the client or event type: '{}'", t)
        }
        Conflict(t: String) {
            description("There are no empty slots for this subscriptions")
            display("There are no empty slots for this subscriptions: '{}'", t)
        }
        InvalidResponse(t: String) {
            description("The response from nakadi made further processing impossible")
            display("The response from nakadi made further processing impossible: '{}'", t)
        }
        CursorUnprocessable(t: String) {
            description("The response from nakadi made further processing impossible")
            display("The response from nakadi made further processing impossible: '{}'", t)
        }
    }
  }