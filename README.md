# Rust + Lambda + DynamoDB

After my [Serverless Hello World](https://github.com/ymwjbxxq/hello_serverless_rust), I wanted to practise more and use DynamoDB.
On this project far to be perfect, I get input and do a more straightforward query to DynamoDB.

### What I have Learnt ###

[AWS SA Nicolas Moutschen](https://twitter.com/NMoutschen) helped me get started and pointed me in the right direction.

AWS SDK is in alpha, so many things can change before the v1 is out. For this example, I used:
```
lambda_runtime = "0.4.1"
aws-config = "0.0.22-alpha"
aws-sdk-dynamodb = "0.0.22-alpha"
aws-types = "0.0.22-alpha"
```
If you run:
```
cargo build --release --target x86_64-unknown-linux-musl
```
You will get this beautiful error:
```
error: failed to run custom build command for `ring v0.16.20`

Caused by:
 process didn't exit successfully: `/Users/fra0005d/git/rust-examples/rust-dynamodb/target/release/build/ring-e4e3d5f387d41c64/build-script-build` (exit status: 101)
 --- stdout
 OPT_LEVEL = Some("3")
 TARGET = Some("x86_64-unknown-linux-musl")
 HOST = Some("x86_64-apple-darwin")
 CC_x86_64-unknown-linux-musl = None
 CC_x86_64_unknown_linux_musl = None
 TARGET_CC = None
 CC = None
 CROSS_COMPILE = None
 CFLAGS_x86_64-unknown-linux-musl = None
 CFLAGS_x86_64_unknown_linux_musl = None
 TARGET_CFLAGS = None
 CFLAGS = None
 CRATE_CC_NO_DEFAULTS = None
 DEBUG = Some("false")
 CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")

 --- stderr
 running "musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I" "include" "-Wall" "-Wextra" "-pedantic" "-pedantic-errors" "-Wall" "-Wextra" "-Wcast-align" "-Wcast-qual" "-Wconversion" "-Wenum-compare" "-Wfloat-equal" "-Wformat=2" "-Winline" "-Winvalid-pch" "-Wmissing-field-initializers" "-Wmissing-include-dirs" "-Wredundant-decls" "-Wshadow" "-Wsign-compare" "-Wsign-conversion" "-Wundef" "-Wuninitialized" "-Wwrite-strings" "-fno-strict-aliasing" "-fvisibility=hidden" "-fstack-protector" "-g3" "-U_FORTIFY_SOURCE" "-DNDEBUG" "-c" "-o/Users/fra0005d/git/rust-examples/rust-dynamodb/target/x86_64-unknown-linux-musl/release/build/ring-e7f51b4231eb5b63/out/aesni-x86_64-elf.o" "/Users/fra0005d/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/pregenerated/aesni-x86_64-elf.S"
 thread 'main' panicked at 'failed to execute ["musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I" "include" "-Wall" "-Wextra" "-pedantic" "-pedantic-errors" "-Wall" "-Wextra" "-Wcast-align" "-Wcast-qual" "-Wconversion" "-Wenum-compare" "-Wfloat-equal" "-Wformat=2" "-Winline" "-Winvalid-pch" "-Wmissing-field-initializers" "-Wmissing-include-dirs" "-Wredundant-decls" "-Wshadow" "-Wsign-compare" "-Wsign-conversion" "-Wundef" "-Wuninitialized" "-Wwrite-strings" "-fno-strict-aliasing" "-fvisibility=hidden" "-fstack-protector" "-g3" "-U_FORTIFY_SOURCE" "-DNDEBUG" "-c" "-o/Users/fra0005d/git/rust-examples/rust-dynamodb/target/x86_64-unknown-linux-musl/release/build/ring-e7f51b4231eb5b63/out/aesni-x86_64-elf.o" "/Users/fra0005d/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/pregenerated/aesni-x86_64-elf.S"]: No such file or directory (os error 2)', /Users/fra0005d/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/build.rs:653:9
 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
```
The error will go away installing [CROSS](https://github.com/rust-embedded/cross). Now the deploy.sh has the command. 
```
cross build --release --target x86_64-unknown-linux-musl
```

From this [Workarounds to Know and Love](https://rust-lang.github.io/async-book/07_workarounds/01_chapter.html)
Rust's async support is still relatively new.
Currently, async fn cannot be used in traits. The reasons for this are somewhat complex, but there are plans to remove this restriction in the future.
**In the meantime, however, this can be worked around using the [async-trait](https://github.com/dtolnay/async-trait) crate from crates.io.**


### TEST - 128 MB - COLD ###

```
START RequestId: 64956f8b-9823-4ddc-aea6-303606076b36 Version: $LATEST
2021-10-27 13:05:42,938 INFO  [bootstrap::library::lambda::handler] input Request { pk: Some("Daniele") }
2021-10-27 13:05:43,078 INFO  [aws_config::meta::region] load_region; provider=EnvironmentVariableRegionProvider { env: Env(Real) }
2021-10-27 13:05:43,138 INFO  [tracing::span] load_config_file
2021-10-27 13:05:43,138 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 13:05:43,138 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/config 
2021-10-27 13:05:43,138 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/config size=0 
2021-10-27 13:05:43,138 INFO  [tracing::span] load_credentials_file
2021-10-27 13:05:43,138 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 13:05:43,138 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/credentials 
2021-10-27 13:05:43,139 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/credentials size=0 
2021-10-27 13:05:43,139 WARN  [aws_config::profile::retry_config] failed to get selected 'default' profile 
2021-10-27 13:05:43,157 INFO  [tracing::span] build_profile_provider
2021-10-27 13:05:43,358 INFO  [bootstrap::library::lambda::handler] Fetching product Daniele
2021-10-27 13:05:43,378 INFO  [aws_smithy_http_tower::parse_response] send_operation
2021-10-27 13:05:43,379 INFO  [aws_smithy_http_tower::parse_response] send_operation; operation="GetItem"
2021-10-27 13:05:43,379 INFO  [aws_smithy_http_tower::parse_response] send_operation; service="dynamodb"
2021-10-27 13:05:43,379 INFO  [aws_config::meta::credentials::chain] load_credentials; provider=Environment
2021-10-27 13:05:43,379 INFO  [aws_config::meta::credentials::chain] loaded credentials provider=Environment 
2021-10-27 13:05:43,447 INFO  [aws_smithy_http_tower::parse_response] send_operation; status="ok"
2021-10-27 13:05:43,447 ERROR [lambda_runtime] InternalError: Missing 'id' 
END RequestId: 64956f8b-9823-4ddc-aea6-303606076b36
REPORT RequestId: 64956f8b-9823-4ddc-aea6-303606076b36  Duration: 510.35 ms Billed Duration: 540 ms Memory Size: 128 MB Max Memory Used: 20 MB  Init Duration: 29.47 ms 
```

### TEST - 128 MB - COLD ###

```
START RequestId: 14ac9a7d-2656-42c3-b3cb-76a1f0024ae6 Version: $LATEST
2021-10-27 14:15:30,808 INFO  [bootstrap::library::lambda::handler] input Request { pk: Some("Daniele") }
2021-10-27 14:15:30,962 INFO  [aws_config::meta::region] load_region; provider=EnvironmentVariableRegionProvider { env: Env(Real) }
2021-10-27 14:15:31,002 INFO  [tracing::span] load_config_file
2021-10-27 14:15:31,002 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:15:31,002 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/config 
2021-10-27 14:15:31,002 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/config size=0 
2021-10-27 14:15:31,002 INFO  [tracing::span] load_credentials_file
2021-10-27 14:15:31,002 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:15:31,002 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/credentials 
2021-10-27 14:15:31,002 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/credentials size=0 
2021-10-27 14:15:31,002 WARN  [aws_config::profile::retry_config] failed to get selected 'default' profile 
2021-10-27 14:15:31,002 INFO  [tracing::span] build_profile_provider
2021-10-27 14:15:31,223 INFO  [bootstrap::library::lambda::handler] Fetching product Daniele
2021-10-27 14:15:31,242 INFO  [aws_smithy_http_tower::parse_response] send_operation
2021-10-27 14:15:31,242 INFO  [aws_smithy_http_tower::parse_response] send_operation; operation="GetItem"
2021-10-27 14:15:31,242 INFO  [aws_smithy_http_tower::parse_response] send_operation; service="dynamodb"
2021-10-27 14:15:31,242 INFO  [aws_config::meta::credentials::chain] load_credentials; provider=Environment
2021-10-27 14:15:31,242 INFO  [aws_config::meta::credentials::chain] loaded credentials provider=Environment 
2021-10-27 14:15:31,318 INFO  [aws_smithy_http_tower::parse_response] send_operation; status="ok"
END RequestId: 14ac9a7d-2656-42c3-b3cb-76a1f0024ae6
REPORT RequestId: 14ac9a7d-2656-42c3-b3cb-76a1f0024ae6  Duration: 512.40 ms Billed Duration: 544 ms Memory Size: 128 MB Max Memory Used: 20 MB  Init Duration: 30.98 ms 
```

### TEST - 128 MB - WARM ###

```
START RequestId: 103e6e42-ce73-41b9-8b07-48680441fa43 Version: $LATEST
2021-10-27 14:15:46,273 INFO  [bootstrap::library::lambda::handler] input Request { pk: Some("Daniele") }
2021-10-27 14:15:46,273 INFO  [aws_config::meta::region] load_region; provider=EnvironmentVariableRegionProvider { env: Env(Real) }
2021-10-27 14:15:46,273 INFO  [tracing::span] load_config_file
2021-10-27 14:15:46,273 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:15:46,273 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/config 
2021-10-27 14:15:46,273 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/config size=0 
2021-10-27 14:15:46,273 INFO  [tracing::span] load_credentials_file
2021-10-27 14:15:46,273 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:15:46,273 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/credentials 
2021-10-27 14:15:46,273 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/credentials size=0 
2021-10-27 14:15:46,273 WARN  [aws_config::profile::retry_config] failed to get selected 'default' profile 
2021-10-27 14:15:46,273 INFO  [tracing::span] build_profile_provider
2021-10-27 14:15:46,274 INFO  [bootstrap::library::lambda::handler] Fetching product Daniele
2021-10-27 14:15:46,274 INFO  [aws_smithy_http_tower::parse_response] send_operation
2021-10-27 14:15:46,274 INFO  [aws_smithy_http_tower::parse_response] send_operation; operation="GetItem"
2021-10-27 14:15:46,274 INFO  [aws_smithy_http_tower::parse_response] send_operation; service="dynamodb"
2021-10-27 14:15:46,274 INFO  [aws_config::meta::credentials::chain] load_credentials; provider=Environment
2021-10-27 14:15:46,274 INFO  [aws_config::meta::credentials::chain] loaded credentials provider=Environment 
2021-10-27 14:15:46,299 INFO  [aws_smithy_http_tower::parse_response] send_operation; status="ok"
END RequestId: 103e6e42-ce73-41b9-8b07-48680441fa43
REPORT RequestId: 103e6e42-ce73-41b9-8b07-48680441fa43  Duration: 29.04 ms  Billed Duration: 30 ms  Memory Size: 128 MB Max Memory Used: 20 MB  
```

### TEST - 1024 MB - COLD ###

```
START RequestId: 7cb70abb-8ca9-48ed-9ca2-b295e902c19f Version: $LATEST
2021-10-27 14:18:00,742 INFO  [bootstrap::library::lambda::handler] input Request { pk: Some("Daniele") }
2021-10-27 14:18:00,755 INFO  [aws_config::meta::region] load_region; provider=EnvironmentVariableRegionProvider { env: Env(Real) }
2021-10-27 14:18:00,759 INFO  [tracing::span] load_config_file
2021-10-27 14:18:00,759 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:18:00,759 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/config 
2021-10-27 14:18:00,759 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/config size=0 
2021-10-27 14:18:00,759 INFO  [tracing::span] load_credentials_file
2021-10-27 14:18:00,759 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:18:00,759 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/credentials 
2021-10-27 14:18:00,759 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/credentials size=0 
2021-10-27 14:18:00,759 WARN  [aws_config::profile::retry_config] failed to get selected 'default' profile 
2021-10-27 14:18:00,759 INFO  [tracing::span] build_profile_provider
2021-10-27 14:18:00,781 INFO  [bootstrap::library::lambda::handler] Fetching product Daniele
2021-10-27 14:18:00,782 INFO  [aws_smithy_http_tower::parse_response] send_operation
2021-10-27 14:18:00,782 INFO  [aws_smithy_http_tower::parse_response] send_operation; operation="GetItem"
2021-10-27 14:18:00,782 INFO  [aws_smithy_http_tower::parse_response] send_operation; service="dynamodb"
2021-10-27 14:18:00,782 INFO  [aws_config::meta::credentials::chain] load_credentials; provider=Environment
2021-10-27 14:18:00,782 INFO  [aws_config::meta::credentials::chain] loaded credentials provider=Environment 
2021-10-27 14:18:00,806 INFO  [aws_smithy_http_tower::parse_response] send_operation; status="ok"
END RequestId: 7cb70abb-8ca9-48ed-9ca2-b295e902c19f
REPORT RequestId: 7cb70abb-8ca9-48ed-9ca2-b295e902c19f  Duration: 65.48 ms  Billed Duration: 104 ms Memory Size: 1024 MB  Max Memory Used: 20 MB  Init Duration: 37.57 ms 
```

### TEST - 1024 MB - WARM ###

```
START RequestId: 00dcc1cf-eac2-4479-8fab-07abaa07061b Version: $LATEST
2021-10-27 14:18:16,051 INFO  [bootstrap::library::lambda::handler] input Request { pk: Some("Daniele") }
2021-10-27 14:18:16,051 INFO  [aws_config::meta::region] load_region; provider=EnvironmentVariableRegionProvider { env: Env(Real) }
2021-10-27 14:18:16,051 INFO  [tracing::span] load_config_file
2021-10-27 14:18:16,051 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:18:16,051 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/config 
2021-10-27 14:18:16,051 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/config size=0 
2021-10-27 14:18:16,051 INFO  [tracing::span] load_credentials_file
2021-10-27 14:18:16,051 WARN  [aws_config::profile::parser::source] could not determine home directory but home expansion was requested 
2021-10-27 14:18:16,051 INFO  [aws_config::profile::parser::source] config file not found path=~/.aws/credentials 
2021-10-27 14:18:16,051 INFO  [aws_config::profile::parser::source] config file loaded path=~/.aws/credentials size=0 
2021-10-27 14:18:16,051 WARN  [aws_config::profile::retry_config] failed to get selected 'default' profile 
2021-10-27 14:18:16,051 INFO  [tracing::span] build_profile_provider
2021-10-27 14:18:16,051 INFO  [bootstrap::library::lambda::handler] Fetching product Daniele
2021-10-27 14:18:16,051 INFO  [aws_smithy_http_tower::parse_response] send_operation
2021-10-27 14:18:16,051 INFO  [aws_smithy_http_tower::parse_response] send_operation; operation="GetItem"
2021-10-27 14:18:16,051 INFO  [aws_smithy_http_tower::parse_response] send_operation; service="dynamodb"
2021-10-27 14:18:16,051 INFO  [aws_config::meta::credentials::chain] load_credentials; provider=Environment
2021-10-27 14:18:16,051 INFO  [aws_config::meta::credentials::chain] loaded credentials provider=Environment 
2021-10-27 14:18:16,083 INFO  [aws_smithy_http_tower::parse_response] send_operation; status="ok"
END RequestId: 00dcc1cf-eac2-4479-8fab-07abaa07061b
REPORT RequestId: 00dcc1cf-eac2-4479-8fab-07abaa07061b  Duration: 33.64 ms  Billed Duration: 34 ms  Memory Size: 1024 MB  Max Memory Used: 20 MB  
```
