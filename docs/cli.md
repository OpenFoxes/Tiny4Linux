# CLI-Documentation

This document contains all available actions and commands for the CLI.

## Sleeping

With these commands you can set the camera to sleep or wake it up.

```shell
t4l turn on
```

| Action | Description               | Command | Aliases    |
| ------ | ------------------------- | ------- | ---------- |
| Sleep  | Sets the camera to sleep. | `sleep` | `turn off` |
| Wake   | Wakes the camera up.      | `wake`  | `turn on`  |

## Tracking Mode

With these commands you can control the AI-tracking of the camera.
You can either turn it completely off or set it to a specific mode.

All tracking commands begin with `tracking` or `track`, e.g.:

```shell
t4l tracking normal
```

| Action      | Description                                                                 | Command      | Aliases          |
| ----------- | --------------------------------------------------------------------------- | ------------ | ---------------- |
| No Tracking | Turns off the AI-tracking and keeps the camera still                        | `static`     | `none`, `off`    |
| Standard    | Turns on the standard AI-tracking, keeping the upper body and face in focus | `normal`     | `standard`, `on` |
| Closeup     | Similar to standard tracking, zoomed closer                                 | `close-up`   | `close`          |
| Upper Body  | Tracks the upper body, similar to standard tracking                         | `upper-body` |                  |
| Headless    | Tracks the upper body without the head                                      | `headless`   |                  |
| Lower Body  | Tracks the lower body                                                       | `lower-body` |                  |
| Desk        | Points the camera at the desk                                               | `desk`       |                  |
| Whiteboard  | Points the camera at a whiteboard                                           | `whiteboard` |                  |
| Hand        | Tracks a hand motion                                                        | `hand`       | `point`          |
| Group       | Tracks a group of people                                                    | `group`      |                  |

## Tracking Speed

With these commands you can control the speed that the camera is tracking with.
You can only use the presets ( `standard` and `sport`).

All tracking-speed commands begin with `speed` or `tracking-speed`, e.g.:

```shell
t4l speed standard
```

| Action   | Description                                          | Command    | Aliases                            |
| -------- | ---------------------------------------------------- | ---------- | ---------------------------------- |
| Standard | Sets the speed to the default, slower tracking speed | `standard` | `normal`, `default`, `slow`, `low` |
| Sport    | Sets the speed to the faster tracking speed          | `fast`     | `sport`, `high`                    |

## Presets

With these commands you can set the position of the camera to one of the 3 possible presets.

The command is `preset` or `position`, e.g.:

```shell
t4l preset 2
```

WARNING: Only the numbers 1 to 3 are valid.

You can set the presets with OBSBOT Center, e.g., running in a virtual machine.

## HDR

You can turn HDR on or off with the command `hdr`.

```shell
t4l hdr on
t4l hdr off
```

## Exposure Mode

With these commands you can control the exposure mode of the camera.

All exposure commands begin with `exposure`, e.g.:

```shell
t4l exposure manual
```

| Action | Description                                       | Command  |
| ------ | ------------------------------------------------- | -------- |
| Manual | Uses custom settings as exposure                  | `manual` |
| Global | Uses global reference for setting the exposure    | `global` |
| Face   | Uses a face as reference for setting the exposure | `face`   |

## Image

These commands change the picture of the camera.

```shell
t4l fov medium
t4l mirror on
```

| Action        | Description                                                           | Command                    | Aliases         |
| ------------- | --------------------------------------------------------------------- | -------------------------- | --------------- |
| Field of View | Sets the field of view                                                | `fov wide\|medium\|narrow` | `field-of-view` |
| Mirror        | Mirrors the image                                                     | `mirror on\|off`           |                 |
| Invert        | Reverses the gimbal's left/right direction; the image does not change | `invert on\|off`           |                 |

The camera does not report the invert setting back, so `info` cannot show it.

## Status LED

With the command `led` you can set the brightness of the status LED (`off`, `1`, `2` or `3`).

```shell
t4l led 2
```

## Auto-Sleep

With the command `auto-sleep` you can set the time after which the camera falls asleep on its own,
or turn auto-sleep off.

```shell
t4l auto-sleep 2m
t4l auto-sleep off
```

Valid times are `off`, seconds (`90` or `30s`) and minutes (`2m`, `10m`).
Turning auto-sleep off keeps the configured time, like OBSBOT Center does.

## Sleep Background

With the command `sleep-background` you can choose what the camera shows while asleep (`black` or the built-in `image`)
and whether that background is mirrored.

```shell
t4l sleep-background image
t4l sleep-background black --mirror off
```

## Audio

With these commands you can control the microphone of the camera.

All audio commands begin with `audio`, e.g.:

```shell
t4l audio noise-reduction medium
```

| Action                  | Description                                        | Command                                     | Aliases      |
| ----------------------- | -------------------------------------------------- | ------------------------------------------- | ------------ |
| Noise Reduction         | Sets the noise reduction of the microphone         | `noise-reduction off\|weak\|medium\|strong` |              |
| Automatic Gain          | Turns the automatic gain on or off                 | `auto-gain on\|off`                         |              |
| Pickup Distance         | Sets the audio pickup distance                     | `pickup-distance near\|standard\|far`       |              |
| Microphone while asleep | Keeps the microphone on while the camera is asleep | `mic-in-sleep on\|off`                      |              |
| Microphone              | Enables or disables the microphone                 | `mic on\|off`                               | `microphone` |

WARNING: Disabling the microphone makes the camera restart. It is unavailable for about 20 seconds.

## Gestures

With these commands you can control the gesture control of the camera.

All gesture commands begin with `gesture` or `gestures`, e.g.:

```shell
t4l gesture zoom on
t4l gesture zoom-ratio 2.5
```

| Action           | Description                              | Command                | Aliases            |
| ---------------- | ---------------------------------------- | ---------------------- | ------------------ |
| Target Lock      | Turns the target lock gesture on or off  | `target-lock on\|off`  |                    |
| Zoom             | Turns the zoom gesture on or off         | `zoom on\|off`         |                    |
| Dynamic Zoom     | Turns the dynamic zoom gesture on or off | `dynamic-zoom on\|off` |                    |
| Direction Change | Mirrors the direction of the gestures    | `direction on\|off`    | `direction-change` |
| Zoom Ratio       | Sets how far the zoom gesture zooms in   | `zoom-ratio <1.0-4.0>` |                    |

The camera only reports the target lock back, so `info` cannot show the other gesture settings.

## Factory Reset

With the command `factory-reset` you can reset all settings of the camera to their factory defaults.
The camera asks for a confirmation unless you pass `--yes`, and restarts afterwards.

```shell
t4l factory-reset
```

## Info

You can display available information about the current state of the camera with the command `info`.

```shell
t4l info
```

## Verbose

With the flag `--verbose` you can enable verbose logging.
You will see the communication with the camera in the console.

## Auto-Completion

If you use the cli more often, it might be useful to enable auto-completion for the commands.
Therefore, you can use the command `completions` to generate the shell-completion scripts for your shell.

In the following example, the shell-completion scripts are generated for bash with `bash-completion`:

```shell
t4l completions bash > /.local/share/bash-completion/completions/t4l
```
