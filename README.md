# The Index Checker

The purpose of Findify Index Checker is to find if index exists for a given merchant as quickly and as efficiently as
possible. This tool is meant to be used before more expensive services such as Lucy are booted.

## Usage

```
Usage: findify-index-checker [OPTIONS] --merchant-id <MERCHANT_ID> --pulsar-service-url <PULSAR_SERVICE_URL> --environment <ENVIRONMENT>

Options:
  -m, --merchant-id <MERCHANT_ID>
          [env: MERCHANT_ID=]
      --pulsar-service-url <PULSAR_SERVICE_URL>
          [env: PULSAR_SERVICE_URL=]
  -e, --environment <ENVIRONMENT>
          [env: ENVIRONMENT=]
  -p, --pulsar-product-topic-template <PULSAR_PRODUCT_TOPIC_TEMPLATE>
          [default: persistent://findify/index-updates-product-{}/{}]
  -t, --timeout <TIMEOUT>
          [default: 5]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example

```bash
./target/debug/findify-index-checker --pulsar-service-url=pulsar://pulsar.us.findify.private:6650 -e prod -m 9795 -t 3
Pulsar service URL: pulsar://pulsar.us.findify.private:6650, Environment: prod, Topic: persistent://findify/index-updates-product-prod/9795, Timeout: 3s
Found index (there could be more or never): 0_4_2024-05-14-135007.425
```

If the index is found, the status code is `0`; otherwise, it is `1` with error messages.

## Development

```bash
cargo run --
```

## Authors

- [Oto Brglez](https://github.com/otobrglez)
