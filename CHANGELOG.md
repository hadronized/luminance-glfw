### 0.4.1

> Monday, 2nd of October, 2017

- Implement `Display` and `Error` for `GLFWDeviceError`.

## 0.4.0

> Sunday, 1st of October, 2017

- Use `luminance-windowing` to benefit from the types and functions defined in there.

### 0.3.3

> Saturday, 30th of September, 2017

- Remove the `luminance` dependency as it’s not needed anymore.

### 0.3.2

- Poll everything!

### 0.3.1

- Support for `glfw-0.16`.

## 0.3.0

- Removed `open_window` and moved its code into `Device::new`.
- Enhanced the documentation.
- Implemented Hi-DPI (tested on a Macbook Pro).
- Removed `Device::width` and `Device::height` and replaced them with `Device::size`.

## 0.2.0

- Changed the way events are handled. It doesn’t create an events thread anymore but instead exposes
  a polling interface. It’ll enhance performance and make mono-thread systems work, like Mac OSX.

### 0.1.5

- Internal fix for Mac OSX.

### 0.1.4

- Updated `luminance` dependencies.

### 0.1.3

- Changed the trait bound from `Fn` to `FnOnce` on `Device::draw`.

### 0.1.2

- Made `glfw::InitError` visible.

### 0.1.1

- `Device::draw` added.

## 0.1.0

- Initial revision.
