# Validation for dev-2026-10-28-maint.1

Validated October 28, 2026. Architecture: Linux amd64 only.

- 304 workspace tests and 5 documentation tests passed.
- The final mDNS run passed all 22 tests, including five UDP regressions for
  the retained empty-response fix and valid Hue discovery responses.
- An initial run hit the inherited `service_daemon::tests::test_refresh_ptr`
  timing assertion. It was also reproduced with the unchanged previous build.
  No tests were skipped or weakened. Final passing counts are not a claim that
  every attempt passed.
- Isolated container HTTP/HTTPS startup and restart checks passed.
- Synthetic upgrade preserved resource IDs, certificate, pairing response,
  bridge identity and an entertainment configuration.
- The same binary was installed and started on an amd64 Home Assistant OS host.
  Configuration and certificate were preserved, as were all 361 saved resource
  IDs and types. Its API exposed 363 resources, including 64 lights and 13 rooms.
- The owner confirmed on/off, brightness and state feedback using the existing
  Hue app pairing with real lights. No re-pairing was necessary.

## Limits

Hue Sync/Entertainment was not separately tested with physical hardware in this
release. No new physical-network mDNS capture was made for the October deployment;
the patch was covered by the isolated UDP regressions and earlier deployment work.
The inherited Zigbee2MQTT resource-not-found log message still occurs for an
unsupported resource. The new upstream room/zone and schedule features were covered
by workspace tests, not every possible real-device scenario.

The dependency audit still reported crypto-bigint RUSTSEC-2026-0387, quick-xml
RUSTSEC-2026-0194 and RUSTSEC-2026-0195, and rsa RUSTSEC-2023-0071. These are
dependency matches, not proof of reachable exploitation. This is not an all-clear
security audit. Container OS package updates are resolved at installation/build
time, so a future container build need not be byte-identical to the tested image.
The released executable itself is the exact tested binary; its SHA256 is recorded.
