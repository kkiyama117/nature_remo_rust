# Unofficial Nature Remo API Sample Implementation (Rust)

## Usage

### Cargo (Rust)

```shell
# In your rust project
cargo add nature_remo_api
```

### Sample

Here is the example of getting user info and update user's nickname and country.

```rust
use nature_remo_api_client::NatureRemoAPIClient;
use nature_remo_api_client::domain::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API Access token from Environment Variables in this time.
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let mut client = NatureRemoAPIClient::new(&access_token);
    let user = client.get_me().await?;
    println!("{:?}", user);
    // Create Post Parameter
    let new_user = UserProfileParam {
        country: Some("JP".to_string()),
        nickname: Some("NickName".to_string()),
        distance_unit: None,
        temp_unit: None
    };
    let user = client.update_user(&new_user).await?;
    println!("{:?}", user);
    Ok(())
}
```

## Details

### crates

- `nature_remo_api`: Provide domain, define request metadata(endpoint and model of parameters) and Python binding.
- `nature_remo_api_client`: Implement of simple client with tokio and hyper.

### Development status

- [x] means `Tested By Real API`
- [0] `Implemented but not tested`

#### Rust

- [0] POST:/1/appliance_orders
- [x] GET:/1/appliances
- [0] POST:/1/appliances
- [0] POST:/1/appliances/{applianceid}
- [0] POST:/1/appliances/{applianceid}/aircon_settings
- [0] POST:/1/appliances/{applianceid}/delete
- [0] POST:/1/appliances/{applianceid}/signal_orders
- [0] GET:/1/appliances/{applianceid}/signals
- [0] POST:/1/appliances/{applianceid}/signals
- ([0] POST:/1/detectappliance)
- [x] GET:/1/devices
- [0] POST:/1/devices/{deviceid}
- [0] POST:/1/devices/{deviceid}/delete
- [x] POST:/1/devices/{deviceid}/humidity_offset
- [x] POST:/1/devices/{deviceid}/temperature_offset
- [x] GET:/1/users/me
- [x] POST:/1/users/me