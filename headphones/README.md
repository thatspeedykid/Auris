# Headphone profiles

This directory contains EQ correction profiles sourced from the
[AutoEQ](https://github.com/jaakkopasanen/AutoEq) project (MIT license).

Each profile corrects the frequency response of a specific headphone model
toward the Harman target curve — the scientifically validated "most preferred"
sound signature.

## Format

Each device has a folder named after the device. Inside:

```
headphones/
  AirPods Pro/
    parametric_eq.txt    # Parametric EQ filter settings
    fr.csv               # Raw frequency response measurement
  Sony WH-1000XM5/
    parametric_eq.txt
    fr.csv
```

`parametric_eq.txt` contains lines like:
```
Filter 1: ON PK Fc 31 Hz Gain -2.1 dB Q 1.41
Filter 2: ON PK Fc 105 Hz Gain 3.4 dB Q 0.71
...
```

## Adding a device

1. Measure your headphones or find measurements from oratory1990, Rtings, or crinacle
2. Run AutoEQ to generate the correction filters
3. Create a folder with the device name
4. Add `parametric_eq.txt` and `fr.csv`
5. Open a PR

## Sources

Measurements come from:
- [oratory1990](https://www.reddit.com/r/oratory1990/wiki/index/headphone_list/)
- [Rtings](https://www.rtings.com)
- [crinacle](https://crinacle.com)
- [Innerfidelity](https://www.innerfidelity.com) (archived)
