# SystemView Tracing
[![no-std](https://github.com/OpenDevicePartnership/systemview-tracing/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/systemview-tracing/actions/workflows/nostd.yml)
[![check](https://github.com/OpenDevicePartnership/systemview-tracing/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/systemview-tracing/actions/workflows/check.yml)
[![LICENSE](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

## Introduction

This is a library that adds support for using Segger SystemView tracing for ODP projects.

## Usage

To get started with adding SystemView tracing to your Embassy application:
  1. Add this crate to your Cargo.toml.
  ```
   systemview-tracing = { git = "https://github.com/OpenDevicePartnership/systemview-tracing", version = "0.1.0" }
  ```
  2. Add this feature and enable the following dependencies:
  ```
  [features]​
  systemview-tracing = [​
      "systemview-tracing/tracing-enabled",​
      "embassy-executor/rtos-trace",​
      "embassy-imxrt/systemview-tracing",​
  ]
  ```

  3. Initialize SystemView tracing by adding the following line to the beginning of your main function
  ```
  systemview_tracing::init_tracing(system_clock_frequency)
  ```
  Note: You can find system_clock_frequency by SYS_CLK_FREQ = CORE_CPU_FREQ / 2

  4. Since the `defmt_rtt` crate cannot be used with this crate simultaneously, make sure to add this config flag before any `defmt_rtt` import:

  ```
  #[cfg(not(feature = "systemview-tracing"))]
  use defmt_rtt as _;
  ```