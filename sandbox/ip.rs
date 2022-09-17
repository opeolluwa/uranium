use std::env;
use std::net::SocketAddr;

fn main() {
    /*
     * if you can read the environment variable value for PORt from the .env
     * parse the read value into the variable value_from_env els use 8405
     */
    let port = env::var("PORT")
        .ok()
        .and_then(|value_from_env| value_from_env.parse().ok())
        .unwrap_or_else(|| 8405);
    /*
     * if there is an env value,
     * try the parse the value to determine of the environment is development or production
     * else, assign the localhost ip address to catch error an fall through
     */

    let ip_address = match env::var("ENVIRONMENT") {
        /*
         * if the environment is production, use the derived port and the placeholder address
         * else use the default localhost IP address and a chosen port
         */
        Ok(env) => {
            if env == String::from("production").trim() {
                //return the placeholder address and the computed port
                SocketAddr::from(([0, 0, 0, 0], port))
            } else {
                //return localhost IP address
                SocketAddr::from(([127, 0, 0, 1], port))
            }
        }

        _ =>
        /*
         * return the localhost IP address as a fall through
         * if the address cannot be found, or badly constructed
         */
        {
            SocketAddr::from(([127, 0, 0, 1], port))
        }
    };
}
